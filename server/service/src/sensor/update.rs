use super::{
    query::{get_sensor, get_sensor_logs_for_breach},
    validate::check_sensor_exists,
};
use crate::{
    activity_log::activity_log_entry, service_provider::ServiceContext, NullableUpdate,
    SingleRecordError,
};
use chrono::{Duration, NaiveDateTime, NaiveTime};
use repository::{
    ActivityLogType, RepositoryError, Sensor, SensorRow, SensorRowRepository, StorageConnection,
    TemperatureBreachRowRepository, TemperatureBreachRowType, TemperatureLog,
    TemperatureLogRowRepository,
};

#[derive(PartialEq, Debug)]
pub enum UpdateSensorError {
    SensorDoesNotExist,
    SensorDoesNotBelongToCurrentStore,
    UpdatedRecordNotFound,
    LocationIsOnHold,
    DatabaseError(RepositoryError),
}

#[derive(Clone)]
pub struct UpdateSensor {
    pub id: String,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub location_id: Option<NullableUpdate<String>>,
    pub log_interval: Option<i32>,
    pub battery_level: Option<i32>,
}

pub fn update_sensor(
    ctx: &ServiceContext,
    input: UpdateSensor,
) -> Result<Sensor, UpdateSensorError> {
    let sensor = ctx
        .connection
        .transaction_sync(|connection| {
            let sensor_row = validate(connection, &ctx.store_id, &input)?;
            let updated_sensor_row = generate(input.clone(), sensor_row.clone());
            SensorRowRepository::new(&connection).upsert_one(&updated_sensor_row)?;

            if let Some(location_update) = input.location_id {
                if sensor_row.location_id == location_update.value {
                    activity_log_entry(
                        ctx,
                        ActivityLogType::SensorLocationChanged,
                        Some(sensor_row.id),
                        sensor_row.location_id,
                        location_update.value,
                    )?;
                }
            }
            get_sensor(ctx, updated_sensor_row.id).map_err(UpdateSensorError::from)
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(sensor)
}

pub fn update_sensor_logs_for_breach(
    connection: &StorageConnection,
    breach_id: &String,
) -> Result<Vec<TemperatureLog>, RepositoryError> {
    let mut temperature_logs: Vec<TemperatureLog> = Vec::new();

    let breach_result =
        TemperatureBreachRowRepository::new(connection).find_one_by_id(breach_id)?;

    if let Some(mut breach_record) = breach_result {
        let is_cumulative_breach = (breach_record.r#type
            == TemperatureBreachRowType::ColdCumulative)
            | (breach_record.r#type == TemperatureBreachRowType::HotCumulative);
        let logs = get_sensor_logs_for_breach(connection, breach_id)?; //sorted by date/time
        let mut log_interval = 0;
        let zero_time = NaiveTime::parse_from_str("00:00", "%H:%M").unwrap(); // hard-coded -> should always work!

        if let Some(sensor) = SensorRowRepository::new(connection).find_one_by_id(&breach_record.sensor_id)? {
            if let Some(interval) = sensor.log_interval {
                log_interval = interval;
            }
        }
        
        if is_cumulative_breach {
            // Update breach start/end from first/last logs if it has changed
            if let Some(first_log) = logs.first() {
                // If within log_interval seconds of the start of the day,
                // use the start of the day as first log time
                let mut first_breach_datetime = NaiveDateTime::new(first_log.temperature_log_row.datetime.date(), zero_time);
                let first_log_datetime = first_breach_datetime + Duration::seconds(log_interval.into());

                if first_log_datetime < first_log.temperature_log_row.datetime {
                    first_breach_datetime = first_log.temperature_log_row.datetime;
                }

                // Use the first log time if it's either earlier than the calculated breach end
                // or if the calculated breach start is less than the sensor log interval before the first log time
                if (breach_record.start_datetime > first_breach_datetime) | (log_interval > 0) & (breach_record.start_datetime < first_breach_datetime - Duration::seconds(log_interval.into())) {
                    log::info!(
                        "Updating cumulative breach start for {:?} to {:?}",
                        breach_record,
                        first_breach_datetime
                    );
                    breach_record.start_datetime = first_breach_datetime;
                    TemperatureBreachRowRepository::new(connection).upsert_one(&breach_record)?;
                }
            }
            if let Some(last_log) = logs.last() {
                // If within log_interval seconds of the end of the day,
                // use the end of the day as last log time
                let mut last_breach_datetime = NaiveDateTime::new(last_log.temperature_log_row.datetime.date(), zero_time) + Duration::days(1);
                let last_log_datetime = last_breach_datetime - Duration::seconds(log_interval.into());

                if last_log_datetime > last_log.temperature_log_row.datetime {
                    last_breach_datetime = last_log.temperature_log_row.datetime;
                }
                
                // Use the last log time if it's either later than the calculated breach end
                // or if the calculated breach end is more than the sensor log interval after the last log time
                if (breach_record.end_datetime < Some(last_breach_datetime)) | (log_interval > 0) & (breach_record.end_datetime > Some(last_breach_datetime + Duration::seconds(log_interval.into()))) {
                    log::info!(
                        "Updating cumulative breach end for {:?} to {:?}",
                        breach_record,
                        last_breach_datetime
                    );
                    breach_record.end_datetime = Some(last_breach_datetime);
                    TemperatureBreachRowRepository::new(connection).upsert_one(&breach_record)?;
                }
            }
        }

        for mut temperature_log in logs {
            if let Some(_breach_id) = &temperature_log.temperature_log_row.temperature_breach_id {
                if is_cumulative_breach {
                    // Skip as cumulative breach can only update unassigned temperature logs
                    continue;
                }
            };

            temperature_log.temperature_log_row.temperature_breach_id = Some(breach_id.to_string());
            TemperatureLogRowRepository::new(connection)
                .upsert_one(&temperature_log.temperature_log_row)?;
            temperature_logs.push(temperature_log.clone());
        }
        log::info!("Temperature logs assigned for breach {:?}", breach_record);

        Ok(temperature_logs)
    } else {
        Err(RepositoryError::NotFound)
    }
}

pub fn validate(
    connection: &StorageConnection,
    store_id: &str,
    input: &UpdateSensor,
) -> Result<SensorRow, UpdateSensorError> {
    let sensor_row = match check_sensor_exists(&input.id, connection)? {
        Some(sensor_row) => sensor_row,
        None => return Err(UpdateSensorError::SensorDoesNotExist),
    };
    if sensor_row.store_id != store_id.to_string() {
        return Err(UpdateSensorError::SensorDoesNotBelongToCurrentStore);
    }

    Ok(sensor_row)
}

pub fn generate(
    UpdateSensor {
        id: _,
        name,
        is_active,
        location_id,
        log_interval,
        battery_level,
    }: UpdateSensor,
    mut sensor_row: SensorRow,
) -> SensorRow {
    // if location has been passed, update sensor_row to the value passed (including if this is null)
    // A null value being passed as the LocationUpdate is the unassignment of location_id
    // no LocationUpdate being passed is the location not being updated
    if let Some(location_id) = location_id {
        sensor_row.location_id = location_id.value;
    }
    sensor_row.name = name.unwrap_or(sensor_row.name);
    sensor_row.is_active = is_active.unwrap_or(sensor_row.is_active);
    sensor_row.log_interval = log_interval.or(sensor_row.log_interval);
    sensor_row.battery_level = battery_level.or(sensor_row.battery_level);
    sensor_row
}

impl From<RepositoryError> for UpdateSensorError {
    fn from(error: RepositoryError) -> Self {
        UpdateSensorError::DatabaseError(error)
    }
}

impl From<SingleRecordError> for UpdateSensorError {
    fn from(error: SingleRecordError) -> Self {
        use UpdateSensorError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => UpdatedRecordNotFound,
        }
    }
}

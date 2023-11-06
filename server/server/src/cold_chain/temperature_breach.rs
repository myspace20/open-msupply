use std::convert::TryInto;

use actix_web::{
    http::header,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use anyhow::Context;
use chrono::NaiveDateTime;
use log::error;
use mime_guess::mime;
use repository::{RepositoryError, TemperatureBreachRowType};
use service::{
    auth_data::AuthData,
    service_provider::{ServiceContext, ServiceProvider},
    temperature_breach::{insert::InsertTemperatureBreach, update::UpdateTemperatureBreach},
    SingleRecordError,
};
use util::constants::SYSTEM_USER_ID;

use super::validate_request;

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureBreach {
    id: String,
    acknowledged: bool,
    #[serde(rename = "endTimestamp")]
    end_unix_timestamp: Option<i64>,
    sensor_id: String,
    #[serde(rename = "startTimestamp")]
    start_unix_timestamp: i64,
    #[serde(rename = "thresholdDuration")]
    pub threshold_duration_milliseconds: i32,
    #[serde(rename = "thresholdMaximumTemperature")]
    pub threshold_maximum: f64,
    #[serde(rename = "thresholdMinimumTemperature")]
    pub threshold_minimum: f64,
    pub r#type: TemperatureBreachRowType,
}

pub async fn put_breaches(
    request: HttpRequest,
    service_provider: Data<ServiceProvider>,
    auth_data: Data<AuthData>,
    breaches: web::Json<Vec<TemperatureBreach>>,
) -> HttpResponse {
    let store_id = match validate_request(request, &service_provider, &auth_data) {
        Ok((_user, store_id)) => store_id,
        Err(error) => {
            let formatted_error = format!("{:#?}", error);
            return HttpResponse::Unauthorized().body(formatted_error);
        }
    };
    let breaches = breaches.into_inner();
    if !validate_input(&breaches) {
        return HttpResponse::BadRequest()
            .body("Expecting a body with the array of temperature breaches");
    };

    let results = match upsert_temperature_breaches(service_provider, store_id, breaches).await {
        Ok(response) => response,
        Err(error) => return HttpResponse::InternalServerError().body(format!("{:#?}", error)),
    };

    HttpResponse::Ok()
        .append_header(header::ContentType(mime::APPLICATION_JSON))
        .json(&results)
}

fn validate_input(breaches: &Vec<TemperatureBreach>) -> bool {
    breaches.iter().all(|breach| validate_breach(breach))
}

fn validate_breach(breach: &TemperatureBreach) -> bool {
    match breach.end_unix_timestamp {
        Some(end_unix_timestamp) => {
            if end_unix_timestamp < 0 {
                return false;
            }
        }
        None => {}
    }
    if breach.start_unix_timestamp < 0 {
        return false;
    }
    if breach.threshold_duration_milliseconds < 0 {
        return false;
    }
    true
}

async fn upsert_temperature_breaches(
    service_provider: Data<ServiceProvider>,
    store_id: String,
    breaches: Vec<TemperatureBreach>,
) -> Result<Vec<Result<repository::TemperatureBreach, String>>, RepositoryError> {
    let ctx = service_provider.context(store_id, SYSTEM_USER_ID.to_string())?;
    let results = breaches
        .into_iter()
        .map(|breach| {
            upsert_temperature_breach(&service_provider, &ctx, breach.clone()).map_err(|e| {
                error!("{:#?} {:?}", e, breach);
                e.to_string()
            })
        })
        .collect();

    Ok(results)
}

fn upsert_temperature_breach(
    service_provider: &ServiceProvider,
    ctx: &ServiceContext,
    breach: TemperatureBreach,
) -> anyhow::Result<repository::TemperatureBreach> {
    let id = breach.id.clone();
    let service = &service_provider.temperature_breach_service;
    let sensor_service = &service_provider.sensor_service;
    let sensor = sensor_service
        .get_sensor(&ctx, breach.sensor_id.clone())
        .map_err(|e| anyhow::anyhow!("Unable to get sensor {:?}", e))?;
    let start_datetime = NaiveDateTime::from_timestamp_opt(breach.start_unix_timestamp, 0)
        .context(format!(
            "Unable to parse timestamp {}",
            breach.start_unix_timestamp
        ))?;

    let duration_milliseconds: i32 = match breach.end_unix_timestamp {
        Some(end_unix_timestamp) => ((end_unix_timestamp - breach.start_unix_timestamp) * 1000)
            .try_into()
            .unwrap_or(0),
        None => 0,
    };

    let end_datetime = match breach.end_unix_timestamp {
        Some(end_unix_timestamp) => NaiveDateTime::from_timestamp_opt(end_unix_timestamp, 0),
        None => None,
    };

    let result = match service.get_temperature_breach(&ctx, id.clone()) {
        Ok(_) => {
            let breach = UpdateTemperatureBreach {
                id: id.clone(),
                location_id: sensor.sensor_row.location_id,
                sensor_id: sensor.sensor_row.id,
                duration_milliseconds,
                r#type: breach.r#type,
                start_datetime,
                end_datetime,
                acknowledged: breach.acknowledged,
                threshold_duration_milliseconds: breach.threshold_duration_milliseconds,
                threshold_maximum: breach.threshold_maximum,
                threshold_minimum: breach.threshold_minimum,
            };
            service
                .update_temperature_breach(&ctx, breach)
                .map_err(|e| anyhow::anyhow!("Unable to update temperature breach {:?}", e))?
        }
        Err(SingleRecordError::NotFound(_)) => {
            let breach = InsertTemperatureBreach {
                id: id.clone(),
                location_id: sensor.sensor_row.location_id,
                sensor_id: sensor.sensor_row.id,
                duration_milliseconds,
                r#type: breach.r#type,
                start_datetime,
                end_datetime,
                acknowledged: breach.acknowledged,
                threshold_duration_milliseconds: breach.threshold_duration_milliseconds,
                threshold_maximum: breach.threshold_maximum,
                threshold_minimum: breach.threshold_minimum,
            };
            service
                .insert_temperature_breach(&ctx, breach)
                .map_err(|e| anyhow::anyhow!("Unable to insert temperature breach {:?}", e))?
        }
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Unable to get temperature breach for id '{}'. {:#?}",
                id.clone(),
                e
            ))
        }
    };

    Ok(result)
}

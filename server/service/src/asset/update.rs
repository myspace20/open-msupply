use super::{
    location::set_asset_location,
    query::get_asset,
    validate::{check_asset_exists, check_locations_are_assigned},
};
use crate::{
    activity_log::activity_log_entry, service_provider::ServiceContext, NullableUpdate,
    SingleRecordError,
};
use chrono::{NaiveDate, Utc};
use repository::{
    assets::{
        asset::{AssetFilter, AssetRepository},
        asset_row::{AssetRow, AssetRowRepository},
    },
    ActivityLogType, EqualFilter, RepositoryError, StorageConnection, StringFilter,
};

#[derive(PartialEq, Debug)]
pub enum UpdateAssetError {
    AssetDoesNotExist,
    AssetDoesNotBelongToCurrentStore,
    SerialNumberAlreadyExists,
    UpdatedRecordNotFound,
    DatabaseError(RepositoryError),
    LocationsAlreadyAssigned,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAsset {
    pub id: String,
    pub store_id: Option<String>,
    pub notes: Option<String>,
    pub asset_number: Option<String>,
    pub serial_number: Option<NullableUpdate<String>>,
    pub catalogue_item_id: Option<NullableUpdate<String>>,
    pub installation_date: Option<NullableUpdate<NaiveDate>>,
    pub replacement_date: Option<NullableUpdate<NaiveDate>>,
    pub location_ids: Option<Vec<String>>,
}

pub fn update_asset(
    ctx: &ServiceContext,
    input: UpdateAsset,
) -> Result<AssetRow, UpdateAssetError> {
    let asset = ctx
        .connection
        .transaction_sync(|connection| {
            let asset_row = validate(connection, &input)?;
            let updated_asset_row = generate(&ctx.store_id, input.clone(), asset_row.clone());
            AssetRowRepository::new(&connection).upsert_one(&updated_asset_row)?;

            activity_log_entry(
                ctx,
                ActivityLogType::AssetUpdated,
                Some(updated_asset_row.id.clone()),
                Some(serde_json::to_string(&asset_row).unwrap_or_default()),
                Some(serde_json::to_string(&updated_asset_row).unwrap_or_default()),
            )?;

            if input.location_ids.clone().is_some() {
                set_asset_location(connection, &asset_row.id, input.location_ids.unwrap())
                    .map_err(|error| UpdateAssetError::DatabaseError(error))?;
            }

            get_asset(ctx, updated_asset_row.id).map_err(UpdateAssetError::from)
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(asset)
}

pub fn validate(
    connection: &StorageConnection,
    input: &UpdateAsset,
) -> Result<AssetRow, UpdateAssetError> {
    let asset_row = match check_asset_exists(&input.id, connection)? {
        Some(asset_row) => asset_row,
        None => return Err(UpdateAssetError::AssetDoesNotExist),
    };

    // Check the serial number is unique (if present)
    if let Some(serial_number) = &input.serial_number {
        if let Some(serial_number) = &serial_number.value {
            if AssetRepository::new(connection)
                .query_one(
                    AssetFilter::new()
                        .id(EqualFilter::not_equal_to(&asset_row.id))
                        .serial_number(StringFilter::equal_to(serial_number)),
                )?
                .is_some()
            {
                return Err(UpdateAssetError::SerialNumberAlreadyExists);
            }
        }
    }

    // Check locations aren't assigned to other assets already
    match &input.location_ids {
        Some(location_ids) => {
            match check_locations_are_assigned(location_ids.to_vec(), &input.id, connection) {
                Ok(locations) => {
                    if locations.len() > 0 {
                        return Err(UpdateAssetError::LocationsAlreadyAssigned);
                    };
                }
                Err(repository_error) => {
                    return Err(UpdateAssetError::DatabaseError(repository_error))
                }
            }
        }
        None => (),
    };

    Ok(asset_row)
}

pub fn generate(
    _ctx_store_id: &str,
    UpdateAsset {
        id: _,
        store_id,
        notes,
        asset_number,
        serial_number,
        catalogue_item_id,
        installation_date,
        replacement_date,
        location_ids: _,
    }: UpdateAsset,
    mut asset_row: AssetRow,
) -> AssetRow {
    asset_row.store_id = store_id;
    asset_row.notes = notes;
    asset_row.asset_number = asset_number.unwrap_or(asset_row.asset_number);

    if let Some(serial_number) = serial_number {
        asset_row.serial_number = serial_number.value;
    }

    if let Some(catalogue_item_id) = catalogue_item_id {
        asset_row.catalogue_item_id = catalogue_item_id.value;
    }

    if let Some(installation_date) = installation_date {
        asset_row.installation_date = installation_date.value;
    }

    if let Some(replacement_date) = replacement_date {
        asset_row.replacement_date = replacement_date.value;
    }

    // Set the modified date time
    asset_row.modified_datetime = Utc::now().naive_utc();

    asset_row
}

impl From<RepositoryError> for UpdateAssetError {
    fn from(error: RepositoryError) -> Self {
        UpdateAssetError::DatabaseError(error)
    }
}

impl From<SingleRecordError> for UpdateAssetError {
    fn from(error: SingleRecordError) -> Self {
        use UpdateAssetError::*;
        match error {
            SingleRecordError::DatabaseError(error) => DatabaseError(error),
            SingleRecordError::NotFound(_) => UpdatedRecordNotFound,
        }
    }
}

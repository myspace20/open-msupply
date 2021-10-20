use crate::{
    database::{
        repository::{
            CentralSyncBufferRepository, CentralSyncCursorRepository, NameStoreJoinRepository,
            RepositoryError, StorageConnectionManager,
        },
        schema::CentralSyncBufferRow,
    },
    server::data::RepositoryRegistry,
    util::sync::{
        translation::{import_sync_records, SyncImportError, TRANSLATION_RECORDS},
        CentralSyncBatch, RemoteSyncBatch, RemoteSyncRecord, SyncConnection, SyncConnectionError,
    },
};

use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CentralSyncError {
    #[error("Failed to pull central sync records")]
    PullCentralSyncRecordsError { source: SyncConnectionError },
    #[error("Failed to update central sync buffer records")]
    UpdateCentralSyncBufferRecordsError { source: RepositoryError },
    #[error("Failed to get central sync cursor record")]
    GetCentralSyncCursorRecordError { source: RepositoryError },
    #[error("Failed to get central sync buffer records")]
    GetCentralSyncBufferRecordsError { source: RepositoryError },
    #[error("Failed to import central sync buffer records")]
    ImportCentralSyncRecordsError { source: SyncImportError },
    #[error("Failed to remove central sync buffer records")]
    RemoveCentralSyncBufferRecordsError { source: RepositoryError },
    #[error("Failed to connect to DB")]
    DBConnectionError { source: RepositoryError },
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct RemoteSyncError {
    msg: &'static str,
    source: anyhow::Error,
}

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Failed to sync central records")]
    CentralSyncError {
        #[from]
        source: CentralSyncError,
    },
    #[error("Failed to sync remote records")]
    RemoteSyncError {
        #[from]
        source: RemoteSyncError,
    },
}

pub struct Synchroniser {
    pub connection: SyncConnection,
}

#[allow(unused_assignments)]
impl Synchroniser {
    pub async fn pull_central_records(
        &mut self,
        registry: &RepositoryRegistry,
    ) -> Result<(), CentralSyncError> {
        let connection = registry
            .get::<StorageConnectionManager>()
            .connection()
            .map_err(|source| CentralSyncError::DBConnectionError { source })?;
        let central_sync_cursor_repository = CentralSyncCursorRepository::new(&connection);
        let central_sync_buffer_repository = CentralSyncBufferRepository::new(&connection);

        let mut cursor: u32 = central_sync_cursor_repository
            .get_cursor()
            .await
            .unwrap_or_else(|_| {
                info!("Initialising new central sync cursor...");
                0
            });

        // Arbitrary batch size.
        const BATCH_SIZE: u32 = 500;

        Ok(loop {
            info!("Pulling central sync records...");
            let sync_batch: CentralSyncBatch = self
                .connection
                .pull_central_records(cursor, BATCH_SIZE)
                .await
                .map_err(|source| CentralSyncError::PullCentralSyncRecordsError { source })?;
            info!("Pulled central sync records");

            if let Some(central_sync_records) = sync_batch.data {
                for central_sync_record in central_sync_records {
                    central_sync_buffer_repository
                        .insert_one_and_update_cursor(&central_sync_record)
                        .await
                        .map_err(|source| {
                            CentralSyncError::UpdateCentralSyncBufferRecordsError { source }
                        })?;
                }
            }

            cursor = central_sync_cursor_repository
                .get_cursor()
                .await
                .map_err(|source| CentralSyncError::GetCentralSyncCursorRecordError { source })?;

            if cursor >= sync_batch.max_cursor - 1 {
                info!("All central sync records pulled successfully");
                break;
            }
        })
    }

    // Hacky method for pulling from sync_queue.
    pub async fn pull_remote_records(&mut self) -> Result<Vec<RemoteSyncRecord>, RemoteSyncError> {
        // TODO: only initialize on initial sync.
        info!("Initialising remote sync records...");
        let mut sync_batch: RemoteSyncBatch = self
            .connection
            .initialise_remote_records()
            .await
            .map_err(|source| RemoteSyncError {
                msg: "Failed to initialise remote sync records",
                source: anyhow::Error::from(source),
            })?;
        info!("Initialised remote sync recordse");

        let mut records: Vec<RemoteSyncRecord> = Vec::new();
        while sync_batch.queue_length > 0 {
            info!("Pulling remote sync records...");
            sync_batch = self
                .connection
                .pull_remote_records()
                .await
                .map_err(|source| RemoteSyncError {
                    msg: "Failed to pull remote sync records",
                    source: anyhow::Error::from(source),
                })?;
            info!("Pulled remote sync records");

            // TODO: acknowledge after integration.
            if let Some(data) = sync_batch.data {
                records.append(&mut data.clone());
                info!("Acknowledging remote sync records...");
                self.connection
                    .acknowledge_remote_records(&records)
                    .await
                    .map_err(|source| RemoteSyncError {
                        msg: "Failed to acknowledge remote sync records",
                        source: anyhow::Error::from(source),
                    })?;
                info!("Acknowledged remote sync records");
            }
        }

        Ok(records)
    }

    async fn integrate_central_records(
        &self,
        registry: &RepositoryRegistry,
    ) -> Result<(), CentralSyncError> {
        let connection = registry
            .get::<StorageConnectionManager>()
            .connection()
            .map_err(|source| CentralSyncError::DBConnectionError { source })?;
        let central_sync_buffer_repository = CentralSyncBufferRepository::new(&connection);

        let mut records: Vec<CentralSyncBufferRow> = Vec::new();
        for table_name in TRANSLATION_RECORDS {
            let mut buffer_rows = central_sync_buffer_repository
                .get_sync_entries(table_name)
                .await
                .map_err(|source| CentralSyncError::GetCentralSyncBufferRecordsError { source })?;
            records.append(&mut buffer_rows);
        }

        import_sync_records(registry, &records)
            .await
            .map_err(|source| CentralSyncError::ImportCentralSyncRecordsError { source })?;

        // TODO needs to be done for M1 as name_store_joins are not synced yet but are required in API
        // these records should actually sync from server in remote sync
        if records.len() > 0 {
            match NameStoreJoinRepository::new(&connection).m1_add() {
                Ok(_) => {}
                Err(_) => {}
            };
        }

        central_sync_buffer_repository
            .remove_all()
            .await
            .map_err(|source| CentralSyncError::RemoveCentralSyncBufferRecordsError { source })?;

        Ok(())
    }

    // fn integrate_remote_records(&self, records: Vec<RemoteSyncRecord>) {
    //     records.iter().for_each(|record| {
    //         info!("Integrated remote sync record {}", record.sync_id);
    //     });
    // }

    pub async fn sync(&mut self, registry: &RepositoryRegistry) -> Result<(), SyncError> {
        info!("Syncing central records...");
        self.pull_central_records(registry).await?;
        info!("Successfully synced central records");

        info!("Integrating central records...");
        self.integrate_central_records(registry).await?;
        info!("Successfully integrated central records");

        // info!("Syncing remote records...");
        // let remote_records = self.pull_remote_records().await?;
        // info!("Successfully pulled remote records");

        // info!("Integrating remote records...");
        // self.integrate_remote_records(remote_records);
        // info!("Successfully integrated remote records");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::{
            repository::{get_repositories, CentralSyncBufferRepository, StorageConnectionManager},
            schema::CentralSyncBufferRow,
        },
        server::data::RepositoryRegistry,
        util::{
            configuration,
            settings::Settings,
            sync::{
                translation::test_data::{
                    check_records_against_database, extract_sync_buffer_rows,
                    item::get_test_item_records, master_list::get_test_master_list_records,
                    master_list_line::get_test_master_list_line_records,
                    master_list_name_join::get_test_master_list_name_join_records,
                    name::get_test_name_records, store::get_test_store_records,
                },
                SyncConnection, Synchroniser,
            },
            test_db,
        },
    };

    #[actix_rt::test]
    async fn test_integrate_central_records() {
        let settings: Settings =
            configuration::get_configuration().expect("Failed to parse configuration settings");

        let sync_connection = SyncConnection::new(&settings.sync);

        let settings = test_db::get_test_settings("omsupply-database-integrate_central_records");

        test_db::setup(&settings.database).await;

        let registry = RepositoryRegistry {
            repositories: get_repositories(&settings).await,
        };

        // use test records with cursors that are out of order
        let mut test_records = Vec::new();
        test_records.append(&mut get_test_name_records());
        test_records.append(&mut get_test_item_records());
        test_records.append(&mut get_test_store_records());
        test_records.append(&mut get_test_master_list_records());
        test_records.append(&mut get_test_master_list_name_join_records());
        test_records.append(&mut get_test_master_list_line_records());

        let central_records: Vec<CentralSyncBufferRow> = extract_sync_buffer_rows(&test_records);
        let connection = registry
            .get::<StorageConnectionManager>()
            .connection()
            .unwrap();
        let central_sync_buffer_repository = CentralSyncBufferRepository::new(&connection);

        central_sync_buffer_repository
            .insert_many(&central_records)
            .expect("Failed to insert central sync records into sync buffer");

        let synchroniser = Synchroniser {
            connection: sync_connection,
        };

        synchroniser
            .integrate_central_records(&registry)
            .await
            .expect("Failed to integrate central records");

        check_records_against_database(&registry, test_records).await;
    }
}

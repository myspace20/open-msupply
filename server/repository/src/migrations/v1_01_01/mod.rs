mod create_indexes;
#[cfg(not(feature = "postgres"))]
mod remove_sqlite_check;

use super::{version::Version, Migration};
use crate::StorageConnection;

pub(crate) struct V1_01_01;

impl Migration for V1_01_01 {
    fn version(&self) -> Version {
        Version::from_str("1.1.1")
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        use crate::migrations::sql;

        // Remove Check
        #[cfg(not(feature = "postgres"))]
        remove_sqlite_check::migrate(connection)?;

        // Indexes
        create_indexes::migrate(connection)?;

        // Remove self-referencing name_store_joins
        sql!(
            connection,
            r#"DELETE
                FROM name_store_join 
                WHERE name_store_join.name_id IN (SELECT name_id FROM store WHERE store.id = name_store_join.store_id);"#
        )?;

        Ok(())
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_1_01_01() {
    use crate::migrations::*;
    use crate::test_db::*;

    let version = V1_01_01.version();

    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(version.clone()),
        ..Default::default()
    })
    .await;

    assert_eq!(get_database_version(&connection), version);
}

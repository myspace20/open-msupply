use super::{
    master_list_name_join::master_list_name_join::dsl::*, master_list_row::master_list,
    name_row::name, StorageConnection,
};

use crate::repository_error::RepositoryError;
use crate::{Delete, Upsert};
use diesel::prelude::*;

table! {
    master_list_name_join (id) {
        id -> Text,
        master_list_id -> Text,
        name_id -> Text,
    }
}

#[derive(Clone, Insertable, Queryable, Debug, PartialEq, Eq, AsChangeset)]
#[table_name = "master_list_name_join"]
pub struct MasterListNameJoinRow {
    pub id: String,
    pub master_list_id: String,
    pub name_id: String,
}

joinable!(master_list_name_join -> master_list (master_list_id));
joinable!(master_list_name_join -> name (name_id));

pub struct MasterListNameJoinRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> MasterListNameJoinRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        MasterListNameJoinRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, row: &MasterListNameJoinRow) -> Result<(), RepositoryError> {
        diesel::insert_into(master_list_name_join)
            .values(row)
            .on_conflict(id)
            .do_update()
            .set(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&self, row: &MasterListNameJoinRow) -> Result<(), RepositoryError> {
        diesel::replace_into(master_list_name_join)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub async fn find_one_by_id(
        &self,
        record_id: &str,
    ) -> Result<MasterListNameJoinRow, RepositoryError> {
        let result = master_list_name_join
            .filter(id.eq(record_id))
            .first(&self.connection.connection)?;
        Ok(result)
    }

    pub fn find_one_by_id_option(
        &self,
        record_id: &str,
    ) -> Result<Option<MasterListNameJoinRow>, RepositoryError> {
        let result = master_list_name_join
            .filter(id.eq(record_id))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn delete(&self, record_id: &str) -> Result<(), RepositoryError> {
        diesel::delete(master_list_name_join.filter(id.eq(record_id)))
            .execute(&self.connection.connection)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MasterListNameJoinRowDelete(pub String);
impl Delete for MasterListNameJoinRowDelete {
    fn delete(&self, con: &StorageConnection) -> Result<(), RepositoryError> {
        MasterListNameJoinRepository::new(con).delete(&self.0)
    }
    // Test only
    fn assert_deleted(&self, con: &StorageConnection) {
        assert_eq!(
            MasterListNameJoinRepository::new(con).find_one_by_id_option(&self.0),
            Ok(None)
        )
    }
}

impl Upsert for MasterListNameJoinRow {
    fn upsert_sync(&self, con: &StorageConnection) -> Result<(), RepositoryError> {
        MasterListNameJoinRepository::new(con).upsert_one(self)
    }

    // Test only
    fn assert_upserted(&self, con: &StorageConnection) {
        assert_eq!(
            MasterListNameJoinRepository::new(con).find_one_by_id_option(&self.id),
            Ok(Some(self.clone()))
        )
    }
}

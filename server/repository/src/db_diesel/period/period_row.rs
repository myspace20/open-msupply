use super::period_row::period::dsl as period_dsl;

use crate::{
    db_diesel::name_link_row::name_link, repository_error::RepositoryError, StorageConnection,
    Upsert,
};

use chrono::NaiveDate;
use diesel::prelude::*;

table! {
    period (id) {
        id -> Text,
        period_schedule_id -> Text,
        name -> Text,
        start_date -> Date,
        end_date -> Date,
    }
}

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Default)]
#[diesel(table_name = period)]
pub struct PeriodRow {
    pub id: String,
    pub period_schedule_id: String,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

allow_tables_to_appear_in_same_query!(period, name_link);

pub struct PeriodRowRepository<'a> {
    connection: &'a mut StorageConnection,
}

impl<'a> PeriodRowRepository<'a> {
    pub fn new(connection: &'a mut StorageConnection) -> Self {
        PeriodRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, row: &PeriodRow) -> Result<(), RepositoryError> {
        diesel::insert_into(period_dsl::period)
            .values(row)
            .on_conflict(period_dsl::id)
            .do_update()
            .set(row)
            .execute(&mut self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&mut self, row: &PeriodRow) -> Result<(), RepositoryError> {
        diesel::replace_into(period_dsl::period)
            .values(row)
            .execute(&mut self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(&mut self, id: &str) -> Result<Option<PeriodRow>, RepositoryError> {
        let result = period_dsl::period
            .filter(period_dsl::id.eq(id))
            .first(&mut self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_many_by_program_schedule_ids(
        &mut self,
        period_schedule_ids: Vec<&str>,
    ) -> Result<Vec<PeriodRow>, RepositoryError> {
        let result = period_dsl::period
            .filter(period_dsl::period_schedule_id.eq_any(period_schedule_ids))
            .load(&mut self.connection.connection)?;
        Ok(result)
    }
}

impl Upsert for PeriodRow {
    fn upsert_sync(&self, con: &mut StorageConnection) -> Result<(), RepositoryError> {
        PeriodRowRepository::new(con).upsert_one(self)
    }

    // Test only
    fn assert_upserted(&self, con: &mut StorageConnection) {
        assert_eq!(
            PeriodRowRepository::new(con).find_one_by_id(&self.id),
            Ok(Some(self.clone()))
        )
    }
}

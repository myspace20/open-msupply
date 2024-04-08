use super::period_schedule_row::period_schedule::dsl as period_schedule_dsl;

use crate::{repository_error::RepositoryError, StorageConnection};

use crate::Upsert;
use diesel::prelude::*;

table! {
    period_schedule (id) {
        id -> Text,
        name -> Text,
    }
}

#[derive(Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Default)]
#[diesel(table_name = period_schedule)]
pub struct PeriodScheduleRow {
    pub id: String,
    pub name: String,
}

pub struct PeriodScheduleRowRepository<'a> {
    connection: &'a mut StorageConnection,
}

impl<'a> PeriodScheduleRowRepository<'a> {
    pub fn new(connection: &'a mut StorageConnection) -> Self {
        PeriodScheduleRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, row: &PeriodScheduleRow) -> Result<(), RepositoryError> {
        diesel::insert_into(period_schedule_dsl::period_schedule)
            .values(row)
            .on_conflict(period_schedule_dsl::id)
            .do_update()
            .set(row)
            .execute(&mut self.connection.connection)?;
        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&mut self, row: &PeriodScheduleRow) -> Result<(), RepositoryError> {
        diesel::replace_into(period_schedule_dsl::period_schedule)
            .values(row)
            .execute(&mut self.connection.connection)?;
        Ok(())
    }

    pub fn find_one_by_id(
        &mut self,
        id: &str,
    ) -> Result<Option<PeriodScheduleRow>, RepositoryError> {
        let result = period_schedule_dsl::period_schedule
            .filter(period_schedule_dsl::id.eq(id))
            .first(&mut self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_one_by_name(
        &mut self,
        name: &str,
    ) -> Result<Option<PeriodScheduleRow>, RepositoryError> {
        let result = period_schedule_dsl::period_schedule
            .filter(period_schedule_dsl::name.eq(name))
            .first(&mut self.connection.connection)
            .optional()?;
        Ok(result)
    }
}

impl Upsert for PeriodScheduleRow {
    fn upsert_sync(&self, con: &mut StorageConnection) -> Result<(), RepositoryError> {
        PeriodScheduleRowRepository::new(con).upsert_one(self)
    }

    // Test only
    fn assert_upserted(&self, con: &mut StorageConnection) {
        assert_eq!(
            PeriodScheduleRowRepository::new(con).find_one_by_id(&self.id),
            Ok(Some(self.clone()))
        )
    }
}

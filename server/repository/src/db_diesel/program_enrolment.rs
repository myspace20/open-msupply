use super::{
    program_enrolment_row::program_enrolment::{self, dsl as program_dsl},
    StorageConnection,
};

use crate::{
    diesel_macros::{apply_date_time_filter, apply_equal_filter, apply_sort, apply_sort_no_case},
    DBType, DatetimeFilter, EqualFilter, Pagination, ProgramEnrolmentRow, ProgramEnrolmentStatus,
    RepositoryError, Sort,
};

use diesel::{dsl::IntoBoxed, prelude::*};

#[derive(Clone)]
pub struct ProgramEnrolmentFilter {
    pub patient_id: Option<EqualFilter<String>>,
    pub enrolment_datetime: Option<DatetimeFilter>,
    pub program_enrolment_id: Option<EqualFilter<String>>,
    pub status: Option<EqualFilter<ProgramEnrolmentStatus>>,
    pub document_type: Option<EqualFilter<String>>,
    pub document_name: Option<EqualFilter<String>>,
    pub context: Option<EqualFilter<String>>,
}

impl ProgramEnrolmentFilter {
    pub fn new() -> ProgramEnrolmentFilter {
        ProgramEnrolmentFilter {
            patient_id: None,
            context: None,
            enrolment_datetime: None,
            program_enrolment_id: None,
            status: None,
            document_type: None,
            document_name: None,
        }
    }

    pub fn context(mut self, filter: EqualFilter<String>) -> Self {
        self.context = Some(filter);
        self
    }

    pub fn patient_id(mut self, filter: EqualFilter<String>) -> Self {
        self.patient_id = Some(filter);
        self
    }

    pub fn enrolment_datetime(mut self, filter: DatetimeFilter) -> Self {
        self.enrolment_datetime = Some(filter);
        self
    }

    pub fn program_enrolment_id(mut self, filter: EqualFilter<String>) -> Self {
        self.program_enrolment_id = Some(filter);
        self
    }

    pub fn status(mut self, filter: EqualFilter<ProgramEnrolmentStatus>) -> Self {
        self.status = Some(filter);
        self
    }

    pub fn document_type(mut self, filter: EqualFilter<String>) -> Self {
        self.document_type = Some(filter);
        self
    }

    pub fn document_name(mut self, filter: EqualFilter<String>) -> Self {
        self.document_name = Some(filter);
        self
    }
}

pub enum ProgramEnrolmentSortField {
    Type,
    PatientId,
    EnrolmentDatetime,
    ProgramEnrolmentId,
    Status,
}

pub type ProgramEnrolment = ProgramEnrolmentRow;

pub type ProgramEnrolmentSort = Sort<ProgramEnrolmentSortField>;

type BoxedProgramEnrolmentQuery = IntoBoxed<'static, program_enrolment::table, DBType>;

fn create_filtered_query<'a>(filter: Option<ProgramEnrolmentFilter>) -> BoxedProgramEnrolmentQuery {
    let mut query = program_dsl::program_enrolment.into_boxed();

    if let Some(ProgramEnrolmentFilter {
        patient_id,
        enrolment_datetime,
        program_enrolment_id,
        status,
        document_type,
        document_name,
        context,
    }) = filter
    {
        apply_equal_filter!(query, patient_id, program_dsl::patient_id);
        apply_equal_filter!(query, context, program_dsl::context);
        apply_date_time_filter!(query, enrolment_datetime, program_dsl::enrolment_datetime);
        apply_equal_filter!(
            query,
            program_enrolment_id,
            program_dsl::program_enrolment_id
        );
        apply_equal_filter!(query, status, program_dsl::status);
        apply_equal_filter!(query, document_type, program_dsl::document_type);
        apply_equal_filter!(query, document_name, program_dsl::document_name);
    }
    query
}

pub struct ProgramEnrolmentRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> ProgramEnrolmentRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        ProgramEnrolmentRepository { connection }
    }

    pub fn count(&self, filter: Option<ProgramEnrolmentFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(
        &self,
        filter: ProgramEnrolmentFilter,
    ) -> Result<Vec<ProgramEnrolment>, RepositoryError> {
        self.query(Pagination::new(), Some(filter), None)
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<ProgramEnrolmentFilter>,
        sort: Option<ProgramEnrolmentSort>,
    ) -> Result<Vec<ProgramEnrolment>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                ProgramEnrolmentSortField::PatientId => {
                    apply_sort!(query, sort, program_dsl::patient_id)
                }
                ProgramEnrolmentSortField::Type => {
                    apply_sort!(query, sort, program_dsl::context)
                }
                ProgramEnrolmentSortField::EnrolmentDatetime => {
                    apply_sort!(query, sort, program_dsl::enrolment_datetime)
                }
                ProgramEnrolmentSortField::ProgramEnrolmentId => {
                    apply_sort!(query, sort, program_dsl::program_enrolment_id)
                }
                ProgramEnrolmentSortField::Status => {
                    apply_sort_no_case!(query, sort, program_dsl::status)
                }
            }
        } else {
            query = query.order(program_dsl::context.asc())
        }

        let result = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64)
            .load::<ProgramEnrolment>(&self.connection.connection)?;

        Ok(result)
    }

    pub fn find_one_by_type_and_patient(
        &self,
        r#type: &str,
        patient_id: &str,
    ) -> Result<Option<ProgramEnrolment>, RepositoryError> {
        Ok(program_dsl::program_enrolment
            .filter(program_dsl::document_type.eq(r#type))
            .filter(program_dsl::patient_id.eq(patient_id))
            .first(&self.connection.connection)
            .optional()?)
    }
}

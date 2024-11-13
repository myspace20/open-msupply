use std::collections::HashMap;

use repository::{
    IndicatorColumnRow, IndicatorColumnRowRepository, IndicatorLineRow, IndicatorLineRowRepository,
    Pagination, ProgramIndicatorFilter, ProgramIndicatorRepository, ProgramIndicatorRow,
    ProgramIndicatorSort, RepositoryError, StorageConnection,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IndicatorLine {
    pub line: IndicatorLineRow,
    pub columns: Vec<IndicatorColumnRow>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ProgramIndicator {
    pub program_indicator: ProgramIndicatorRow,
    pub lines: Vec<IndicatorLine>,
}

pub fn program_indicators(
    connection: &StorageConnection,
    pagination: Pagination,
    sort: Option<ProgramIndicatorSort>,
    filter: Option<ProgramIndicatorFilter>,
) -> Result<Vec<ProgramIndicator>, RepositoryError> {
    let indicators = ProgramIndicatorRepository::new(connection).query(pagination, filter, sort)?;

    let indicator_ids: Vec<String> = indicators
        .iter()
        .map(|indicator| indicator.id.clone())
        .collect();

    let mut indicator_line_rows =
        IndicatorLineRowRepository::new(connection).find_many_by_indicator_ids(&indicator_ids)?;
    let mut indicator_column_rows =
        IndicatorColumnRowRepository::new(connection).find_many_by_indicator_ids(&indicator_ids)?;

    let mut result_indicators = Vec::new();

    for program_indicator in indicators.into_iter() {
        let (this_indicator_line_rows, remainder) = indicator_line_rows
            .into_iter()
            .partition(|l| l.program_indicator_id == program_indicator.id);
        indicator_line_rows = remainder;

        let (this_indicator_columns_rows, remainder) = indicator_column_rows
            .into_iter()
            .partition(|l| l.program_indicator_id == program_indicator.id);
        indicator_column_rows = remainder;

        result_indicators.push(ProgramIndicator {
            program_indicator,
            lines: this_indicator_line_rows
                .into_iter()
                .map(|line| IndicatorLine {
                    line,
                    columns: this_indicator_columns_rows.clone(),
                })
                .collect(),
        });
    }

    Ok(result_indicators)
}

#[cfg(test)]
mod query {
    use repository::Pagination;
    use repository::{mock::MockDataInserts, test_db::setup_all};

    use crate::service_provider::ServiceProvider;
    #[actix_rt::test]
    async fn program_indicator_query() {
        let (_, connection, connection_manager, _) =
            setup_all("test_program_indicator_query", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let service = service_provider.program_indicator_service;

        // test mapping of data to graphql structure

        let result = service
            .program_indicators(
                &connection,
                Pagination {
                    limit: 500,
                    offset: 0,
                },
                None,
                None,
            )
            .unwrap();

        // Check finding 2 mock active program indicators
        assert_eq!(result.len(), 2);

        let lines_a = result.get_key_value("program_indicator_a");
        assert_eq!(lines_a.unwrap().1.lines.len(), 3);

        let lines_b = result.get_key_value("program_indicator_b");
        assert_eq!(lines_b.unwrap().1.lines.len(), 1);

        // Check columns are mapped to each line in program_indicator_a
        let columns_a = lines_a
            .unwrap()
            .1
            .lines
            .iter()
            .flat_map(|line| line.columns.iter())
            .count();
        assert_eq!(columns_a, 6);
    }
}

use async_graphql::*;
use graphql_core::generic_filters::{
    DatetimeFilterInput, EqualFilterBigNumberInput, EqualFilterStringInput,
};
use graphql_core::pagination::PaginationInput;
use graphql_core::simple_generic_errors::{
    ErrorWrapper, NodeError, NodeErrorInterface, RecordNotFound,
};
use graphql_core::standard_graphql_error::{
    list_error_to_gql_err, validate_auth, StandardGraphqlError,
};
use graphql_core::{map_filter, ContextExt};
use graphql_types::types::{StocktakeNode, StocktakeNodeStatus};
use repository::StocktakeFilter;
use repository::*;
use repository::{StocktakeSort, StocktakeSortField};
use service::permission_validation::Resource;
use service::permission_validation::ResourceAccessRequest;
use service::ListError;

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(rename_items = "camelCase")]
pub enum StocktakeSortFieldInput {
    Status,
    CreatedDatetime,
    FinalisedDatetime,
}

#[derive(InputObject)]
pub struct StocktakeSortInput {
    /// Sort query result by `key`
    key: StocktakeSortFieldInput,
    /// Sort query result is sorted descending or ascending (if not provided the default is
    /// ascending)
    desc: Option<bool>,
}

#[derive(InputObject, Clone)]
pub struct EqualFilterStocktakeStatusInput {
    pub equal_to: Option<StocktakeNodeStatus>,
    pub equal_any: Option<Vec<StocktakeNodeStatus>>,
    pub not_equal_to: Option<StocktakeNodeStatus>,
}

#[derive(InputObject, Clone)]
pub struct StocktakeFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub stocktake_number: Option<EqualFilterBigNumberInput>,
    pub status: Option<EqualFilterStocktakeStatusInput>,
    pub created_datetime: Option<DatetimeFilterInput>,
    pub finalised_datetime: Option<DatetimeFilterInput>,
}

#[derive(SimpleObject)]
pub struct StocktakeConnector {
    total_count: u32,
    nodes: Vec<StocktakeNode>,
}

#[derive(Union)]
pub enum StocktakesResponse {
    Response(StocktakeConnector),
}

pub fn stocktakes(
    ctx: &Context<'_>,
    store_id: &str,
    page: Option<PaginationInput>,
    filter: Option<StocktakeFilterInput>,
    sort: Option<Vec<StocktakeSortInput>>,
) -> Result<StocktakesResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryStocktake,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_ctx = service_provider.context()?;
    let service = &service_provider.stocktake_service;

    match service.get_stocktakes(
        &service_ctx,
        store_id,
        page.map(PaginationOption::from),
        filter.map(StocktakeFilter::from),
        // Currently only one sort option is supported, use the first from the list.
        sort.and_then(|mut sort_list| sort_list.pop())
            .map(|sort| sort.to_domain()),
    ) {
        Ok(stocktakes) => Ok(StocktakesResponse::Response(StocktakeConnector {
            total_count: stocktakes.count,
            nodes: stocktakes
                .rows
                .into_iter()
                .map(|stocktake| StocktakeNode { stocktake })
                .collect(),
        })),
        Err(err) => {
            let formatted_error = format!("{:#?}", err);
            let graphql_error = match err {
                ListError::DatabaseError(err) => err.into(),
                ListError::LimitBelowMin(_) => StandardGraphqlError::BadUserInput(formatted_error),
                ListError::LimitAboveMax(_) => StandardGraphqlError::BadUserInput(formatted_error),
            };
            Err(graphql_error.extend())
        }
    }
}

#[derive(Union)]
pub enum StocktakeResponse {
    Response(StocktakeNode),
    Error(NodeError),
}

pub fn stocktake(ctx: &Context<'_>, store_id: &str, id: &str) -> Result<StocktakeResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryStocktake,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_ctx = service_provider.context()?;
    let service = &service_provider.stocktake_service;

    match service.get_stocktakes(
        &service_ctx,
        store_id,
        None,
        Some(StocktakeFilter::new().id(EqualFilter::equal_to(id))),
        None,
    ) {
        Ok(mut stocktakes) => {
            let result = match stocktakes.rows.pop() {
                Some(stocktake) => StocktakeResponse::Response(StocktakeNode { stocktake }),
                None => StocktakeResponse::Error(ErrorWrapper {
                    error: NodeErrorInterface::RecordNotFound(RecordNotFound {}),
                }),
            };
            Ok(result)
        }

        Err(err) => Err(list_error_to_gql_err(err)),
    }
}

pub fn stocktake_by_number(
    ctx: &Context<'_>,
    store_id: &str,
    stocktake_number: i64,
) -> Result<StocktakeResponse> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryStocktake,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_ctx = service_provider.context()?;
    let service = &service_provider.stocktake_service;

    match service.get_stocktakes(
        &service_ctx,
        store_id,
        None,
        Some(StocktakeFilter::new().stocktake_number(EqualFilter::equal_to_i64(stocktake_number))),
        None,
    ) {
        Ok(mut stocktakes) => {
            let result = match stocktakes.rows.pop() {
                Some(stocktake) => StocktakeResponse::Response(StocktakeNode { stocktake }),
                None => StocktakeResponse::Error(ErrorWrapper {
                    error: NodeErrorInterface::RecordNotFound(RecordNotFound {}),
                }),
            };
            Ok(result)
        }

        Err(err) => Err(list_error_to_gql_err(err)),
    }
}

impl StocktakeSortInput {
    pub fn to_domain(self) -> StocktakeSort {
        use StocktakeSortField as to;
        use StocktakeSortFieldInput as from;
        let key = match self.key {
            from::Status => to::Status,
            from::CreatedDatetime => to::CreatedDatetime,
            from::FinalisedDatetime => to::FinalisedDatetime,
        };

        StocktakeSort {
            key,
            desc: self.desc,
        }
    }
}

impl From<StocktakeFilterInput> for StocktakeFilter {
    fn from(f: StocktakeFilterInput) -> Self {
        StocktakeFilter {
            id: f.id.map(EqualFilter::from),
            store_id: None,
            stocktake_number: f.stocktake_number.map(EqualFilter::from),
            status: f
                .status
                .map(|t| map_filter!(t, StocktakeNodeStatus::to_domain)),
            created_datetime: f.created_datetime.map(DatetimeFilter::from),
            finalised_datetime: f.finalised_datetime.map(DatetimeFilter::from),
        }
    }
}

#[cfg(test)]
mod test {
    use async_graphql::EmptyMutation;
    use chrono::NaiveDate;
    use graphql_core::assert_graphql_query;
    use graphql_core::test_helpers::setup_graphl_test;
    use repository::PaginationOption;
    use repository::{
        mock::{mock_stocktake_a, MockDataInserts},
        Stocktake, StocktakeFilter, StocktakeRow, StocktakeSort, StocktakeStatus,
        StorageConnectionManager,
    };
    use serde_json::json;
    use service::{
        service_provider::{ServiceContext, ServiceProvider},
        stocktake::StocktakeServiceTrait,
        ListError, ListResult,
    };
    use util::inline_init;

    use crate::StocktakeQueries;

    type ServiceMethod = dyn Fn(
            &ServiceContext,
            &str,
            Option<PaginationOption>,
            Option<StocktakeFilter>,
            Option<StocktakeSort>,
        ) -> Result<ListResult<Stocktake>, ListError>
        + Sync
        + Send;

    pub struct TestService(pub Box<ServiceMethod>);

    impl StocktakeServiceTrait for TestService {
        fn get_stocktakes(
            &self,
            ctx: &ServiceContext,
            store_id: &str,
            pagination: Option<PaginationOption>,
            filter: Option<StocktakeFilter>,
            sort: Option<StocktakeSort>,
        ) -> Result<ListResult<Stocktake>, ListError> {
            (self.0)(ctx, store_id, pagination, filter, sort)
        }
    }

    pub fn service_provider(
        test_service: TestService,
        connection_manager: &StorageConnectionManager,
    ) -> ServiceProvider {
        let mut service_provider = ServiceProvider::new(connection_manager.clone());
        service_provider.stocktake_service = Box::new(test_service);
        service_provider
    }

    #[actix_rt::test]
    async fn test_graphql_stocktakes_query() {
        let (_, _, connection_manager, settings) = setup_graphl_test(
            StocktakeQueries,
            EmptyMutation,
            "omsupply-database-gql-stocktakes_query",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"query QueryStocktakes($storeId: String, $page: PaginationInput!, $filter: StocktakeFilterInput, $sort: [StocktakeSortInput]) {
            stocktakes(storeId: $storeId, page: $page, filter: $filter, sort: $sort) {
                ... on StocktakeConnector {
                    totalCount
                    nodes {
                        id
                        storeId
                        stocktakeNumber
                        comment
                        description
                        status
                        createdDatetime
                        stocktakeDate
                        finalisedDatetime
                        isLocked
                        inventoryAdjustmentId
                        lines {
                            totalCount
                        }
                    }                      
                }
            }
        }"#;

        // success
        let test_service = TestService(Box::new(|_, _, _, _, _| {
            Ok(ListResult {
                count: 1,
                rows: vec![inline_init(|r: &mut StocktakeRow| {
                    r.id = "id1".to_string();
                    r.stocktake_number = 123;
                    r.store_id = "store id".to_string();
                    r.comment = Some("comment".to_string());
                    r.description = Some("description".to_string());
                    r.status = StocktakeStatus::Finalised;
                    r.created_datetime = NaiveDate::from_ymd(2022, 1, 22).and_hms(15, 16, 0);
                    r.stocktake_date = Some(NaiveDate::from_ymd(2022, 1, 23));
                    r.is_locked = true;
                    r.finalised_datetime =
                        Some(NaiveDate::from_ymd(2022, 1, 23).and_hms(15, 16, 0));
                    r.inventory_adjustment_id = Some("inv id".to_string());
                })],
            })
        }));
        let variables = Some(json!({
            "storeId": "store id",
            "page": {}
        }));
        let expected = json!({
          "stocktakes": {
            "totalCount": 1,
            "nodes": [{
              "id": "id1",
              "storeId": "store id",
              "stocktakeNumber": 123,
              "comment": "comment",
              "description": "description",
              "status": "FINALISED",
              "createdDatetime": "2022-01-22T15:16:00+00:00",
              "stocktakeDate": "2022-01-23",
              "finalisedDatetime": "2022-01-23T15:16:00+00:00",
              "inventoryAdjustmentId": "inv id",
              "isLocked": true,
              "lines": {
                "totalCount": 0
              }
            }]
          }
        });
        assert_graphql_query!(
            &settings,
            query,
            &variables,
            &expected,
            Some(service_provider(test_service, &connection_manager))
        );
    }

    #[actix_rt::test]
    async fn test_graphql_stocktake_query() {
        let (_, _, _, settings) = setup_graphl_test(
            StocktakeQueries,
            EmptyMutation,
            "omsupply-database-gql-stocktake_query",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"query QueryStocktake($storeId: String, $id: String) {
            stocktake(storeId: $storeId, id: $id) {
                ... on StocktakeNode {
                    id
                }
            }
        }"#;
        let expected_stocktake = mock_stocktake_a();
        let variables = Some(json!({
            "storeId": expected_stocktake.store_id,
            "id": expected_stocktake.id,
        }));
        let expected = json!({
            "stocktake": {
                "id": expected_stocktake.id
            }
        });
        assert_graphql_query!(&settings, query, &variables, &expected, None);
    }

    #[actix_rt::test]
    async fn test_graphql_stocktake_by_number_query() {
        let (_, _, _, settings) = setup_graphl_test(
            StocktakeQueries,
            EmptyMutation,
            "omsupply-database-gql-stocktake_by_number_query",
            MockDataInserts::all(),
        )
        .await;

        let query = r#"query QueryStocktakeByNumber($storeId: String, $stocktakeNumber: String) {
            stocktakeByNumber(storeId: $storeId, stocktakeNumber: $stocktakeNumber) {
                ... on StocktakeNode {
                    stocktakeNumber
                }
            }
        }"#;
        let expected_stocktake = mock_stocktake_a();
        let variables = Some(json!({
            "storeId": expected_stocktake.store_id,
            "stocktakeNumber": expected_stocktake.stocktake_number,
        }));
        let expected = json!({
          "stocktakeByNumber": {
            "stocktakeNumber": expected_stocktake.stocktake_number
          }
        });
        assert_graphql_query!(&settings, query, &variables, &expected, None);
    }
}

use async_graphql::*;

pub mod delete;
pub mod insert;
pub mod update;

pub struct NotAServiceItem;
#[Object]
impl NotAServiceItem {
    pub async fn description(&self) -> &'static str {
        "Not a service item"
    }
}

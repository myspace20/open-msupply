use repository::asset_class_row::AssetClassRow;
use serde_json::json;

use super::{TestSyncPullRecord, TestSyncPushRecord};

const TABLE_NAME: &'static str = "asset_class";

const ASSET_CLASS1: (&'static str, &'static str) = (
    "32608ef9-dce5-41a7-b3e9-92b0fe086c7e",
    r#"{
        "id": "32608ef9-dce5-41a7-b3e9-92b0fe086c7e",
        "name": "Asset Class 1"
    }"#,
);

fn asset_class1() -> AssetClassRow {
    AssetClassRow {
        id: ASSET_CLASS1.0.to_string(),
        name: "Asset Class 1".to_string(),
    }
}

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncPullRecord> {
    vec![TestSyncPullRecord::new_pull_upsert(
        TABLE_NAME,
        ASSET_CLASS1,
        asset_class1(),
    )]
}

pub(crate) fn test_omsupply_central_push_records() -> Vec<TestSyncPushRecord> {
    vec![TestSyncPushRecord {
        table_name: TABLE_NAME.to_string(),
        record_id: ASSET_CLASS1.0.to_string(),
        push_data: json!(asset_class1()),
    }]
}

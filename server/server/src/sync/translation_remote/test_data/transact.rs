use chrono::{Duration, NaiveDate, NaiveTime};
use repository::schema::{
    ChangelogAction, ChangelogRow, ChangelogTableName, InvoiceRow, InvoiceRowStatus,
    InvoiceRowType, RemoteSyncBufferAction, RemoteSyncBufferRow,
};
use serde_json::json;

use crate::sync::translation_remote::{
    invoice::{LegacyTransactRow, LegacyTransactStatus, LegacyTransactType, TransactMode},
    pull::{IntegrationRecord, IntegrationUpsertRecord},
    test_data::TestSyncRecord,
    TRANSLATION_RECORD_TRANSACT,
};

use super::TestSyncPushRecord;

const TRANSACT_1: (&'static str, &'static str) = (
    "12e889c0f0d211eb8dddb54df6d741bc",
    r#"{
      "Colour": 0,
      "Date_order_received": "0000-00-00",
      "Date_order_written": "2021-07-30",
      "ID": "12e889c0f0d211eb8dddb54df6d741bc",
      "amount_outstanding": 0,
      "arrival_date_actual": "0000-00-00",
      "arrival_date_estimated": "0000-00-00",
      "authorisationStatus": "",
      "budget_period_ID": "",
      "category2_ID": "",
      "category_ID": "",
      "comment": "",
      "confirm_date": "2021-07-30",
      "confirm_time": 47046,
      "contact_id": "",
      "currency_ID": "8009D512AC0E4FD78625E3C8273B0171",
      "currency_rate": 1,
      "custom_data": null,
      "diagnosis_ID": "",
      "donor_default_id": "",
      "encounter_id": "",
      "entry_date": "2021-07-30",
      "entry_time": 47046,
      "export_batch": 0,
      "foreign_currency_total": 0,
      "goodsReceivedConfirmation": null,
      "goods_received_ID": "",
      "hold": false,
      "insuranceDiscountAmount": 0,
      "insuranceDiscountRate": 0,
      "internalData": null,
      "invoice_num": 1,
      "invoice_printed_date": "0000-00-00",
      "is_authorised": false,
      "is_cancellation": false,
      "lastModifiedAt": 1627607293,
      "linked_goods_received_ID": "",
      "linked_transaction_id": "",
      "local_charge_distributed": 0,
      "mode": "store",
      "mwks_sequence_num": 0,
      "nameInsuranceJoinID": "",
      "name_ID": "name_store_a",
      "number_of_cartons": 0,
      "optionID": "",
      "original_PO_ID": "",
      "paymentTypeID": "",
      "pickslip_printed_date": "0000-00-00",
      "prescriber_ID": "",
      "requisition_ID": "",
      "responsible_officer_ID": "",
      "service_descrip": "",
      "service_price": 0,
      "ship_date": "0000-00-00",
      "ship_method_ID": "",
      "ship_method_comment": "",
      "status": "cn",
      "store_ID": "store_a",
      "subtotal": 0,
      "supplier_charge_fc": 0,
      "tax": 0,
      "their_ref": "",
      "total": 0,
      "type": "si",
      "user1": "",
      "user2": "",
      "user3": "",
      "user4": "",
      "user_ID": "",
      "wardID": "",
      "waybill_number": ""
  }"#,
);
fn transact_1_pull_record() -> TestSyncRecord {
    TestSyncRecord {
        translated_record: Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::Invoice(InvoiceRow {
                id: TRANSACT_1.0.to_string(),
                user_id: None,
                store_id: "store_a".to_string(),
                name_id: "name_store_a".to_string(),
                name_store_id: Some("store_a".to_string()),
                invoice_number: 1,
                r#type: InvoiceRowType::InboundShipment,
                status: InvoiceRowStatus::Delivered,
                on_hold: false,
                comment: None,
                their_reference: None,
                transport_reference: None,
                created_datetime: NaiveDate::from_ymd(2021, 7, 30).and_hms(0, 0, 0)
                    + Duration::seconds(47046),
                allocated_datetime: None,
                picked_datetime: None,
                shipped_datetime: None,
                delivered_datetime: Some(
                    NaiveDate::from_ymd(2021, 7, 30).and_hms(0, 0, 0) + Duration::seconds(47046),
                ),
                verified_datetime: None,
                colour: Some("#000000".to_string()),
                requisition_id: None,
                linked_invoice_id: None,
            }),
        )),
        identifier: "Transact 1",
        remote_sync_buffer_row: RemoteSyncBufferRow {
            id: "Transact_10".to_string(),
            table_name: TRANSLATION_RECORD_TRANSACT.to_string(),
            record_id: TRANSACT_1.0.to_string(),
            data: TRANSACT_1.1.to_string(),
            action: RemoteSyncBufferAction::Update,
        },
    }
}
fn transact_1_push_record() -> TestSyncPushRecord {
    TestSyncPushRecord {
        change_log: ChangelogRow {
            id: 2,
            table_name: ChangelogTableName::Invoice,
            row_id: TRANSACT_1.0.to_string(),
            row_action: ChangelogAction::Upsert,
        },
        push_data: json!(LegacyTransactRow {
            ID: TRANSACT_1.0.to_string(),
            user_id: None,
            name_ID: "name_store_a".to_string(),
            store_ID: "store_a".to_string(),
            invoice_num: 1,
            _type: LegacyTransactType::Si,
            status: LegacyTransactStatus::Cn,
            hold: false,
            comment: None,
            their_ref: None,
            transport_reference: None,
            Colour: 0,
            requisition_ID: None,
            linked_transaction_id: None,
            entry_date: NaiveDate::from_ymd(2021, 7, 30),
            entry_time: NaiveTime::from_hms(13, 4, 6),
            ship_date: None,
            arrival_date_actual: Some(NaiveDate::from_ymd(2021, 7, 30)),
            confirm_date: Some(NaiveDate::from_ymd(2021, 7, 30)),
            confirm_time: NaiveTime::from_hms(13, 4, 6),
            mode: TransactMode::Store,
        }),
    }
}

const TRANSACT_2: (&'static str, &'static str) = (
    "7c860d40f3f111eb9647790fe8518386",
    r#"{
        "Colour": 1710361,
        "Date_order_received": "0000-00-00",
        "Date_order_written": "2021-08-03",
        "ID": "7c860d40f3f111eb9647790fe8518386",
        "amount_outstanding": 0,
        "arrival_date_actual": "0000-00-00",
        "arrival_date_estimated": "0000-00-00",
        "authorisationStatus": "",
        "budget_period_ID": "",
        "category2_ID": "",
        "category_ID": "",
        "comment": "",
        "confirm_date": "0000-00-00",
        "confirm_time": 44806,
        "contact_id": "",
        "currency_ID": "8009D512AC0E4FD78625E3C8273B0171",
        "currency_rate": 1,
        "custom_data": null,
        "diagnosis_ID": "",
        "donor_default_id": "",
        "encounter_id": "",
        "entry_date": "2021-08-03",
        "entry_time": 44806,
        "export_batch": 0,
        "foreign_currency_total": 0,
        "goodsReceivedConfirmation": null,
        "goods_received_ID": "",
        "hold": false,
        "insuranceDiscountAmount": 0,
        "insuranceDiscountRate": 0,
        "internalData": null,
        "invoice_num": 4,
        "invoice_printed_date": "0000-00-00",
        "is_authorised": false,
        "is_cancellation": false,
        "lastModifiedAt": 1627959832,
        "linked_goods_received_ID": "",
        "linked_transaction_id": "",
        "local_charge_distributed": 0,
        "mode": "store",
        "mwks_sequence_num": 0,
        "nameInsuranceJoinID": "",
        "name_ID": "name_store_b",
        "number_of_cartons": 0,
        "optionID": "",
        "original_PO_ID": "",
        "paymentTypeID": "",
        "pickslip_printed_date": "0000-00-00",
        "prescriber_ID": "",
        "requisition_ID": "",
        "responsible_officer_ID": "",
        "service_descrip": "",
        "service_price": 0,
        "ship_date": "0000-00-00",
        "ship_method_ID": "",
        "ship_method_comment": "",
        "status": "fn",
        "store_ID": "store_b",
        "subtotal": 0,
        "supplier_charge_fc": 0,
        "tax": 0,
        "their_ref": "",
        "total": 0,
        "type": "ci",
        "user1": "",
        "user2": "",
        "user3": "",
        "user4": "",
        "user_ID": "0763E2E3053D4C478E1E6B6B03FEC207",
        "wardID": "",
        "waybill_number": "",
        "om_transport_reference": "transport reference"
    }"#,
);
fn transact_2_pull_record() -> TestSyncRecord {
    TestSyncRecord {
        translated_record: Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::Invoice(InvoiceRow {
                id: TRANSACT_2.0.to_string(),
                user_id: Some("0763E2E3053D4C478E1E6B6B03FEC207".to_string()),
                store_id: "store_b".to_string(),
                name_id: "name_store_b".to_string(),
                name_store_id: Some("store_b".to_string()),
                invoice_number: 4,
                r#type: InvoiceRowType::OutboundShipment,
                status: InvoiceRowStatus::Verified,
                on_hold: false,
                comment: None,
                their_reference: None,
                transport_reference: Some("transport reference".to_string()),
                created_datetime: NaiveDate::from_ymd(2021, 8, 3).and_hms(0, 0, 0)
                    + Duration::seconds(44806),
                allocated_datetime: None,
                picked_datetime: None,
                shipped_datetime: None,
                delivered_datetime: None,
                verified_datetime: None,
                colour: Some("#1A1919".to_string()),
                requisition_id: None,
                linked_invoice_id: None,
            }),
        )),
        identifier: "Transact 2",
        remote_sync_buffer_row: RemoteSyncBufferRow {
            id: "Transact_20".to_string(),
            table_name: TRANSLATION_RECORD_TRANSACT.to_string(),
            record_id: TRANSACT_2.0.to_string(),
            data: TRANSACT_2.1.to_string(),
            action: RemoteSyncBufferAction::Update,
        },
    }
}
fn transact_2_push_record() -> TestSyncPushRecord {
    TestSyncPushRecord {
        change_log: ChangelogRow {
            id: 2,
            table_name: ChangelogTableName::Invoice,
            row_id: TRANSACT_2.0.to_string(),
            row_action: ChangelogAction::Upsert,
        },
        push_data: json!(LegacyTransactRow {
            ID: TRANSACT_2.0.to_string(),
            user_id: Some("0763E2E3053D4C478E1E6B6B03FEC207".to_string()),
            name_ID: "name_store_b".to_string(),
            store_ID: "store_b".to_string(),
            invoice_num: 4,
            _type: LegacyTransactType::Ci,
            status: LegacyTransactStatus::Fn,
            hold: false,
            comment: None,
            their_ref: None,
            transport_reference: Some("transport reference".to_string()),
            Colour: 1710361,
            requisition_ID: None,
            linked_transaction_id: None,
            entry_date: NaiveDate::from_ymd(2021, 8, 3),
            entry_time: NaiveTime::from_hms(12, 26, 46),
            ship_date: None,
            arrival_date_actual: None,
            confirm_date: None,
            // Note: we are loosing this value when date is None
            confirm_time: NaiveTime::from_hms(0, 0, 0),
            mode: TransactMode::Store,
        }),
    }
}

#[allow(dead_code)]
pub fn get_test_transact_records() -> Vec<TestSyncRecord> {
    vec![transact_1_pull_record(), transact_2_pull_record()]
}

#[allow(dead_code)]
pub fn get_test_push_transact_records() -> Vec<TestSyncPushRecord> {
    vec![transact_1_push_record(), transact_2_push_record()]
}

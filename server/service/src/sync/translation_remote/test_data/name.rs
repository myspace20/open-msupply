use chrono::NaiveDate;
use repository::{
    ChangelogAction, ChangelogRow, ChangelogTableName, Gender, NameRow, SyncBufferRow,
};
use serde_json::json;
use util::inline_init;

use crate::sync::{
    translation_central::{LegacyNameRow, LegacyNameType},
    translation_remote::{
        pull::{IntegrationRecord, IntegrationUpsertRecord},
        TRANSLATION_RECORD_NAME,
    },
};

use super::{TestSyncPushRecord, TestSyncRecord};

const NAME_1: (&'static str, &'static str) = (
    "C3FB3B30A8D04DDF9AF59A15BB48668A",
    r#"{
      "ID": "C3FB3B30A8D04DDF9AF59A15BB48668A",
      "name": "Moemoe, Alex",
      "fax": "",
      "phone": "02345678",
      "customer": true,
      "bill_address1": "Bikenibeu",
      "bill_address2": "Marakei",
      "supplier": false,
      "charge code": "00102/19/01",
      "margin": 0,
      "comment": "name comment 1",
      "currency_ID": "8009D512AC0E4FD78625E3C8273B0171",
      "country": "NZ",
      "freightfac": 0,
      "email": "email@some.com",
      "custom1": "",
      "code": "00102/19/00",
      "last": "Moemoe",
      "first": "Alex",
      "title": "",
      "female": true,
      "date_of_birth": "1998-07-29",
      "overpayment": 0,
      "group_ID": "",
      "hold": false,
      "ship_address1": "",
      "ship_address2": "",
      "url": "web1",
      "barcode": "b000000000989",
      "postal_address1": "",
      "postal_address2": "",
      "category1_ID": "8C4DDF227AFB4FD6A09445C949079597",
      "region_ID": "",
      "type": "patient",
      "price_category": "A",
      "flag": "",
      "manufacturer": false,
      "print_invoice_alphabetical": false,
      "custom2": "",
      "custom3": "",
      "default_order_days": 0,
      "connection_type": 0,
      "PATIENT_PHOTO": "data:image/png;base64,",
      "NEXT_OF_KIN_ID": "",
      "POBOX": "",
      "ZIP": 0,
      "middle": "",
      "preferred": false,
      "Blood_Group": "",
      "marital_status": "",
      "Benchmark": false,
      "next_of_kin_relative": "",
      "mother_id": "",
      "postal_address3": "",
      "postal_address4": "",
      "bill_address3": "",
      "bill_address4": "",
      "ship_address3": "",
      "ship_address4": "",
      "ethnicity_ID": "",
      "occupation_ID": "",
      "religion_ID": "",
      "national_health_number": "",
      "Master_RTM_Supplier_Code": 0,
      "ordering_method": "",
      "donor": false,
      "latitude": 0,
      "longitude": 0,
      "Master_RTM_Supplier_name": "",
      "category2_ID": "",
      "category3_ID": "",
      "category4_ID": "",
      "category5_ID": "",
      "category6_ID": "",
      "bill_address5": "",
      "bill_postal_zip_code": "",
      "postal_address5": "",
      "postal_zip_code": "",
      "ship_address5": "",
      "ship_postal_zip_code": "",
      "supplying_store_id": "store_a",
      "license_number": "",
      "license_expiry": "0000-00-00",
      "has_current_license": false,
      "custom_data": null,
      "maximum_credit": 0,
      "nationality_ID": "",
      "created_date": "2022-05-22",
      "integration_ID": ""
  }"#,
);
fn name_1_pull_record() -> TestSyncRecord {
    TestSyncRecord {
        translated_record: Some(IntegrationRecord::from_upsert(
            IntegrationUpsertRecord::Name(NameRow {
                id: NAME_1.0.to_string(),
                name: "Moemoe, Alex".to_string(),
                code: "00102/19/00".to_string(),
                r#type: repository::NameType::Patient,
                is_customer: true,
                is_supplier: false,
                supplying_store_id: Some("store_a".to_string()),
                first_name: Some("Alex".to_string()),
                last_name: Some("Moemoe".to_string()),
                gender: Some(Gender::Female),
                date_of_birth: Some(NaiveDate::from_ymd(1998, 07, 29)),
                phone: Some("02345678".to_string()),
                charge_code: Some("00102/19/01".to_string()),
                comment: Some("name comment 1".to_string()),
                country: Some("NZ".to_string()),
                address1: Some("Bikenibeu".to_string()),
                address2: Some("Marakei".to_string()),
                email: Some("email@some.com".to_string()),
                website: Some("web1".to_string()),
                is_manufacturer: false,
                is_donor: false,
                on_hold: false,
                created_datetime: Some(NaiveDate::from_ymd(2022, 05, 22).and_hms(0, 0, 0)),
            }),
        )),
        identifier: "Name 1",
        remote_sync_buffer_row: inline_init(|r: &mut SyncBufferRow| {
            r.table_name = TRANSLATION_RECORD_NAME.to_string();
            r.record_id = NAME_1.0.to_string();
            r.data = NAME_1.1.to_string();
        }),
    }
}
fn name_1_push_record() -> TestSyncPushRecord {
    TestSyncPushRecord {
        change_log: ChangelogRow {
            id: 2,
            table_name: ChangelogTableName::Name,
            row_id: NAME_1.0.to_string(),
            row_action: ChangelogAction::Upsert,
        },
        push_data: json!(LegacyNameRow {
            ID: NAME_1.0.to_string(),
            name: "Moemoe, Alex".to_string(),
            code: "00102/19/00".to_string(),
            r#type: LegacyNameType::Patient,
            customer: true,
            supplier: false,
            supplying_store_id: Some("store_a".to_string()),
            first_name: Some("Alex".to_string()),
            last_name: Some("Moemoe".to_string()),
            female: true,
            date_of_birth: Some(NaiveDate::from_ymd(1998, 07, 29)),
            phone: Some("02345678".to_string()),
            charge_code: Some("00102/19/01".to_string()),
            comment: Some("name comment 1".to_string()),
            country: Some("NZ".to_string()),
            address1: Some("Bikenibeu".to_string()),
            address2: Some("Marakei".to_string()),
            email: Some("email@some.com".to_string()),
            website: Some("web1".to_string()),
            is_manufacturer: false,
            is_donor: false,
            on_hold: false,
            created_date: Some(NaiveDate::from_ymd(2022, 05, 22)),
            //om_created_datetime: Some(NaiveDate::from_ymd(2022, 05, 22).and_hms(0, 0, 0)),
            //om_gender: Some(Gender::Female),
        }),
    }
}

#[allow(dead_code)]
pub fn get_test_name_records() -> Vec<TestSyncRecord> {
    vec![name_1_pull_record()]
}

#[allow(dead_code)]
pub fn get_test_push_name_records() -> Vec<TestSyncPushRecord> {
    vec![name_1_push_record()]
}

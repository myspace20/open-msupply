use repository::{
    MasterListNameJoinRow, MasterListNameJoinRowDelete, StorageConnection, SyncBufferRow,
};

use serde::Deserialize;

use crate::sync::translations::{master_list::MasterListTranslation, name::NameTranslation};

use super::{PullTranslateResult, SyncTranslation};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct LegacyListMasterNameJoinRow {
    ID: String,
    name_ID: String,
    list_master_ID: String,
}
// Needs to be added to all_translators()
#[deny(dead_code)]
pub(crate) fn boxed() -> Box<dyn SyncTranslation> {
    Box::new(MasterListNameJoinTranslation)
}

pub(super) struct MasterListNameJoinTranslation;
impl SyncTranslation for MasterListNameJoinTranslation {
    fn table_name(&self) -> &'static str {
        "list_master_name_join"
    }

    fn pull_dependencies(&self) -> Vec<&'static str> {
        vec![
            NameTranslation.table_name(),
            MasterListTranslation.table_name(),
        ]
    }

    fn try_translate_pull_upsert(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<PullTranslateResult, anyhow::Error> {
        let data = serde_json::from_str::<LegacyListMasterNameJoinRow>(&sync_record.data)?;
        if data.name_ID == "" {
            return Ok(PullTranslateResult::Ignored("Missing name_id".to_string()));
        }

        let result = MasterListNameJoinRow {
            id: data.ID,
            master_list_id: data.list_master_ID,
            name_id: data.name_ID,
        };

        Ok(PullTranslateResult::upsert(result))
    }

    fn try_translate_pull_delete(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<PullTranslateResult, anyhow::Error> {
        Ok(PullTranslateResult::delete(MasterListNameJoinRowDelete(
            sync_record.record_id.clone(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::{mock::MockDataInserts, test_db::setup_all};

    #[actix_rt::test]
    async fn test_master_list_name_join_translation() {
        use crate::sync::test::test_data::master_list_name_join as test_data;
        let translator = MasterListNameJoinTranslation {};

        let (_, connection, _, _) = setup_all(
            "test_master_list_name_join_translation",
            MockDataInserts::none(),
        )
        .await;

        for record in test_data::test_pull_upsert_records() {
            assert!(translator.match_pull(&record.sync_buffer_row));
            let translation_result = translator
                .try_translate_pull_upsert(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }

        for record in test_data::test_pull_delete_records() {
            assert!(translator.match_pull(&record.sync_buffer_row));
            let translation_result = translator
                .try_translate_pull_delete(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }
    }
}

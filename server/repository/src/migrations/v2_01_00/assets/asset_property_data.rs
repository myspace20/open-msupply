use crate::migrations::constants::{
    COLD_CHAIN_EQUIPMENT_UUID, COLD_ROOMS_AND_FREEZER_ROOMS_UUID, INSULATED_CONTAINERS_UUID,
    REFRIGERATORS_AND_FREEZERS_UUID,
};
use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    // Create the external_dimensions property as an example (available for all cold chain equipment assets)
    sql!(
        connection,
        r#"
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id) VALUES ('external_dimensions', 'external_dimensions', 'External dimensions - WxDxH (in cm)', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}');
        "#,
    )?;

    /*
    Asset Properties for Cold/Freezer rooms (asset_category_id: 7db32eb6-5929-4dd1-a5e9-01e36baa73ad)
    Storage capacity +5 °C (litres)
    Storage capacity -20 °C (litres)
    Storage capacity -70 °C (litres)
    Waterpack storage capacity (Kg)
    Waterpack freezing capacity per 24 hours (Kg)
    Energy consumption (stable running, continuous power) (KW per day)
    Energy consumption during freezing (KW per day)
    Hold over time (hours)
    Climate zone
    Freeze protection
    Temperature monitoring device (integrated, external or none)
    Voltage stabilizer (integrated, external or none)
    Energy Source
    Refrigerant Type(s)
    */

    sql!(
        connection,
        r#"
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_5c-cr', 'storage_capacity_5c', 'Storage capacity +5 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_20c-cr', 'storage_capacity_20c', 'Storage capacity -20 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_70c-cr', 'storage_capacity_70c', 'Storage capacity -70 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('waterpack_storage_capacity-cr', 'waterpack_storage_capacity', 'Waterpack storage capacity (Kg)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('waterpack_freezing_capacity-cr', 'waterpack_freezing_capacity', 'Waterpack freezing capacity per 24 hours (Kg)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_consumption_stable-cr', 'energy_consumption_stable', 'Energy consumption (stable running, continuous power) (KW per day)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_consumption_freezing-cr', 'energy_consumption_freezing', 'Energy consumption during freezing (KW per day)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('hold_over_time-cr', 'hold_over_time', 'Hold over time (hours)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('climate_zone-cr', 'climate_zone', 'Climate zone', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('freeze_protection-cr', 'freeze_protection', 'Freeze protection', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('temperature_monitoring_device-cr', 'temperature_monitoring_device', 'Temperature monitoring device', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('voltage_stabilizer-cr', 'voltage_stabilizer', 'Voltage stabilizer', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_source-cr', 'energy_source', 'Energy Source', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('refrigerant_type-cr', 'refrigerant_type', 'Refrigerant Type(s)', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{COLD_ROOMS_AND_FREEZER_ROOMS_UUID}');
        "#,
    )?;

    /*
    Asset Properties for Fridge/Freezer rooms (asset_category_id: {02cbea92-d5bf-4832-863b-c04e093a7760})
    Storage capacity +5 °C (litres)
    Storage capacity -20 °C (litres)
    Storage capacity -70 °C (litres)
    Waterpack storage capacity (Kg)
    Waterpack freezing capacity per 24 hours (Kg)
    Energy consumption (stable running, continuous power) ((KW) per day)
    Energy consumption during freezing ((KW) per day)
    Hold over time (hours)
    Climate zone
    Freeze protection
    Temperature monitoring device (integrated, external or none)
    Voltage stabilizer (integrated, external or none)
    Energy Source
    Refrigerant Type(s)
    */

    sql!(
        connection,
        r#"
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_5c-fr', 'storage_capacity_5c', 'Storage capacity +5 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_20c-fr', 'storage_capacity_20c', 'Storage capacity -20 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('storage_capacity_70c-fr', 'storage_capacity_70c', 'Storage capacity -70 °C (litres)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('waterpack_storage_capacity-fr', 'waterpack_storage_capacity', 'Waterpack storage capacity (Kg)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('waterpack_freezing_capacity-fr', 'waterpack_freezing_capacity', 'Waterpack freezing capacity per 24 hours (Kg)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_consumption_stable-fr', 'energy_consumption_stable', 'Energy consumption (stable running, continuous power) (KW per day)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_consumption_freezing-fr', 'energy_consumption_freezing', 'Energy consumption during freezing (KW per day)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('hold_over_time-fr', 'hold_over_time', 'Hold over time (hours)', 'FLOAT', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('climate_zone-fr', 'climate_zone', 'Climate zone', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('freeze_protection-fr', 'freeze_protection', 'Freeze protection', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('temperature_monitoring_device-fr', 'temperature_monitoring_device', 'Temperature monitoring device', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('voltage_stabilizer-fr', 'voltage_stabilizer', 'Voltage stabilizer', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('energy_source-fr', 'energy_source', 'Energy Source', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('refrigerant_type-fr', 'refrigerant_type', 'Refrigerant Type(s)', 'STRING', NULL, '{COLD_CHAIN_EQUIPMENT_UUID}','{REFRIGERATORS_AND_FREEZERS_UUID}');
        "#,
    )?;

    /*
        Asset Properties for Insulated Containers (asset_category_id: b7eea921-5a14-44cc-b5e0-ea59f2e9cb8d)
        Temperature monitoring device (integrated, external or none)
        Voltage stabilizer (integrated, external or none)
    */

    sql!(
        connection,
        r#"
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('temperature_monitoring_device-ic', 'temperature_monitoring_device', 'Temperature monitoring device', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{INSULATED_CONTAINERS_UUID}');
        INSERT INTO asset_property (id, key, name, value_type, allowed_values, asset_class_id, asset_category_id) VALUES ('voltage_stabilizer-ic', 'voltage_stabilizer', 'Voltage stabilizer', 'STRING', 'Integrated, External, None', '{COLD_CHAIN_EQUIPMENT_UUID}','{INSULATED_CONTAINERS_UUID}');
        "#,
    )?;

    // Setup the PQS properties for the asset catalogue items
    sql!(
        connection,
        r#"
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R404A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/001-C';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R404A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/001-F';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R134A, R452A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/002-C';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R134A, R452A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/002-F';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R448A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/003-C';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R448A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/003-F';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R404A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/004-C';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R404A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/004-F';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R134A, R407A, R452A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/005-C';
UPDATE asset_catalogue_item SET properties = '{{"refrigerant_type": "R134A, R407A, R452A", "climate_zone": "Hot, Temperate, Cold", "energy_source": "Electricity"}}' WHERE code = 'E001/005-F';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 121.0, "refrigerant_type": "R134A", "external_dimensions": "82 x 67 x 63", "waterpack_storage_capacity": 81.6, "waterpack_freezing_capacity": 12.0, "energy_consumption_stable": 0.38, "energy_consumption_freezing": 3.77, "hold_over_time": 2.5, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/002';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 298.0, "refrigerant_type": "R134A", "external_dimensions": "81.8 x 124 x 63", "waterpack_storage_capacity": 186.0, "waterpack_freezing_capacity": 16.8, "energy_consumption_stable": 4.36, "hold_over_time": 4.15, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/003';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 105.0, "refrigerant_type": "R134A", "external_dimensions": "84 x 69 x 126", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 3.2, "hold_over_time": 25.6, "climate_zone": "Hot, Temperate, Cold", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/007';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 75.0, "refrigerant_type": "R134A", "external_dimensions": "84 x 70 x 92", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.89, "hold_over_time": 20.1, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/011';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 48.0, "refrigerant_type": "R134A", "external_dimensions": "88 x 96.5 x 71", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 2.3, "hold_over_time": 43.13, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/022';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 281.0, "refrigerant_type": "R600A", "external_dimensions": "84 x 156 x 70", "waterpack_storage_capacity": 153.6, "waterpack_freezing_capacity": 7.2, "energy_consumption_stable": 4.23, "energy_consumption_freezing": 4.24, "hold_over_time": 4.0, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/023';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 105.0, "refrigerant_type": "R600A", "external_dimensions": "84 x 72 x 70", "waterpack_storage_capacity": 38.4, "waterpack_freezing_capacity": 7.2, "energy_consumption_stable": 2.24, "energy_consumption_freezing": 3.33, "hold_over_time": 2.8, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/024';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 171.0, "refrigerant_type": "R600A", "external_dimensions": "84 x 113 x 70", "waterpack_storage_capacity": 96.0, "waterpack_freezing_capacity": 7.2, "energy_consumption_stable": 3.0, "energy_consumption_freezing": 3.56, "hold_over_time": 2.9, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/025';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 156.0, "refrigerant_type": "R600A", "external_dimensions": "91 x 127 x 78", "waterpack_storage_capacity": 9.6, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 94.08, "climate_zone": "Temperate", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/030';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 99.0, "storage_capacity_20c": 42.0, "refrigerant_type": "R600A", "external_dimensions": "127 x 78 x 91", "waterpack_storage_capacity": 14.4, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 94.0, "climate_zone": "Temperate", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/035';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 93.0, "refrigerant_type": "R600A", "external_dimensions": "180 x 85 x 73", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 125.0, "climate_zone": "Temperate", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/037';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 132.0, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/040';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 36.0, "storage_capacity_20c": 4.8, "refrigerant_type": "R600A", "external_dimensions": "90 x 78 x 103", "waterpack_storage_capacity": 3.6, "waterpack_freezing_capacity": 1.89, "energy_consumption_stable": 0.0, "hold_over_time": 94.4, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/042';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 70.0, "storage_capacity_20c": 42.0, "refrigerant_type": "R600A", "external_dimensions": "91 x 127 x 78", "waterpack_storage_capacity": 10.5, "waterpack_freezing_capacity": 2.5, "energy_consumption_stable": 0.0, "hold_over_time": 79.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/043';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 128.0, "refrigerant_type": "R134A", "external_dimensions": "190 x 85 x 72", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.98, "energy_consumption_freezing": 2.03, "hold_over_time": 128.2, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/044';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 89.0, "refrigerant_type": "R600A", "external_dimensions": "91 x 127 x 78", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 124.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/045';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 102.0, "storage_capacity_20c": 42.9, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_storage_capacity": 8.1, "waterpack_freezing_capacity": 2.04, "energy_consumption_stable": 0.0, "hold_over_time": 83.7, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/048';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 46.5, "refrigerant_type": "R600A", "external_dimensions": "121.5 x 79.5 x75", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 119.2, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/049';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 99.0, "refrigerant_type": "R600A", "external_dimensions": "182 x 79.5 x 75", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/050';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 27.0, "refrigerant_type": "R134A", "external_dimensions": "102.8 x 61.9 x 56.3", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.68, "energy_consumption_freezing": 2.56, "hold_over_time": 77.2, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/051';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 128.0, "refrigerant_type": "R134A", "external_dimensions": "189 x 83 x 71", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 167.9, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/052';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 27.0, "refrigerant_type": "R600A", "external_dimensions": "102.5 x 56 x 60", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 87.27, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/055';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 100.0, "storage_capacity_20c": 40.0, "refrigerant_type": "R600A", "external_dimensions": "169.5 x 86.5 x 82.5", "waterpack_storage_capacity": 10.68, "waterpack_freezing_capacity": 2.08, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/057';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 110.0, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 91.65, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/058';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 88.0, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 1.65, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/059';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 96.0, "refrigerant_type": "R600A", "external_dimensions": "79 X 59.5 X 880", "waterpack_storage_capacity": 19.4, "waterpack_freezing_capacity": 14.5, "energy_consumption_stable": 3.31, "energy_consumption_freezing": 2.81, "hold_over_time": 6.73, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/060';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 240.0, "refrigerant_type": "R134A", "external_dimensions": "122.6 x 79 x 945", "waterpack_storage_capacity": 44.3, "waterpack_freezing_capacity": 38.3, "energy_consumption_stable": 3.37, "energy_consumption_freezing": 3.54, "hold_over_time": 58.6, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/061';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 240.0, "refrigerant_type": "R600A", "external_dimensions": "91.5 x 162.5 x 78", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.85, "hold_over_time": 77.3, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/066';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 16.0, "refrigerant_type": "R600A", "external_dimensions": "95 x 73 x 73", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 7.7, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/067';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 36.0, "refrigerant_type": "R600A", "external_dimensions": "90 x 103 78", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 93.4, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/068';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 25.5, "refrigerant_type": "R600A", "external_dimensions": "83 x 55.5 x 64.5", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "hold_over_time": 91.28, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/069';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 52.5, "storage_capacity_20c": 5.1, "refrigerant_type": "R600A", "external_dimensions": "84.5 x 72.5 x 71", "waterpack_storage_capacity": 3.6, "waterpack_freezing_capacity": 1.6, "energy_consumption_stable": 0.63, "energy_consumption_freezing": 1.8, "hold_over_time": 45.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/070';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 204.0, "refrigerant_type": "R290", "external_dimensions": "91 x 127 x 78", "waterpack_storage_capacity": 97.2, "waterpack_freezing_capacity": 32.4, "energy_consumption_stable": 2.15, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/071';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 184.0, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 803.0, "hold_over_time": 94.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/072';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 64.0, "refrigerant_type": "R600A", "external_dimensions": "90 x 103 78", "waterpack_storage_capacity": 11.24, "waterpack_freezing_capacity": 2.16, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/073';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 37.5, "storage_capacity_20c": 32.0, "refrigerant_type": "R600A", "external_dimensions": "72 x 87.5 x 112.8", "waterpack_storage_capacity": 12.0, "waterpack_freezing_capacity": 2.43, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/074';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 22.5, "refrigerant_type": "R600A", "external_dimensions": "72 x 87.5 x 78.8", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/075';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 59.0, "refrigerant_type": "R600A", "external_dimensions": "72 x 87.5 x 112.8", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/076';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 16.0, "storage_capacity_20c": 2.4, "refrigerant_type": "R600A", "external_dimensions": "95 x 73 x 73", "waterpack_storage_capacity": 2.4, "waterpack_freezing_capacity": 1.97, "energy_consumption_stable": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/077';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 52.5, "refrigerant_type": "R600A", "external_dimensions": "97.5 x 100 x 74", "waterpack_freezing_capacity": 0.0, "hold_over_time": 74.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/078';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 50.0, "refrigerant_type": "R290", "external_dimensions": "158.8 x 54.5 x 65.5", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.54, "hold_over_time": 120.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/079';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 51.0, "refrigerant_type": "R600A", "external_dimensions": "151.5 x 61.8 x 77.4", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.63, "hold_over_time": 89.72, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/080';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 72.5, "refrigerant_type": "R600A", "external_dimensions": "151.5 x 61.8 x 77.4", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.47, "hold_over_time": 81.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/081';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 98.5, "refrigerant_type": "R600A", "external_dimensions": "170 x 61.8 x 77.4", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.23, "hold_over_time": 59.56, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/082';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 225.0, "refrigerant_type": "R600A", "external_dimensions": "183 x 79.5 x 75", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 2.04, "hold_over_time": 55.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/083';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 57.0, "storage_capacity_20c": 24.0, "refrigerant_type": "R600A", "external_dimensions": "98 x 128.2 x 74", "waterpack_storage_capacity": 13.8, "waterpack_freezing_capacity": 2.4, "hold_over_time": 94.18, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/084';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 25.5, "refrigerant_type": "R600A", "external_dimensions": "97.5 x 89 x 74", "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/085';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 48.0, "refrigerant_type": "R600A", "external_dimensions": "72 x 87.5 x 78.8", "waterpack_storage_capacity": 20.0, "waterpack_freezing_capacity": 2.4, "hold_over_time": 120.0, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Solar"}}' WHERE code = 'E003/086';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 211.0, "refrigerant_type": "R600A", "external_dimensions": "94 x 164.7 x 71.7", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.62, "hold_over_time": 62.23, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/087';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 122.0, "refrigerant_type": "R600A", "external_dimensions": "87.2 x 112.8 x 71.7", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.54, "hold_over_time": 60.83, "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/088';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 61.0, "refrigerant_type": "R600A", "external_dimensions": "87.2 x 112.8 x 71.7", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.57, "hold_over_time": 59.85, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/089';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 24.2, "refrigerant_type": "R600A", "external_dimensions": "113 x 78 x 85", "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/090';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 20.0, "storage_capacity_20c": 34.3, "refrigerant_type": "R600A", "external_dimensions": "88 x 111 x 65", "waterpack_storage_capacity": 17.4, "waterpack_freezing_capacity": 1.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/091';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 36.0, "storage_capacity_20c": 34.3, "refrigerant_type": "R600A", "external_dimensions": "88 x 111 x 65", "waterpack_storage_capacity": 17.4, "waterpack_freezing_capacity": 1.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/092';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 220.0, "refrigerant_type": "R600A", "external_dimensions": "91.5 x 162.5 x 78", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/093';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 58.0, "storage_capacity_20c": 44.0, "refrigerant_type": "R290", "external_dimensions": "183 x 79.5 x 75", "waterpack_storage_capacity": 14.4, "waterpack_freezing_capacity": 2.4, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/095';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 77.0, "refrigerant_type": "R134A", "external_dimensions": "167 x 85 x 71", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.4, "hold_over_time": 105.28, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/096';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 30.0, "storage_capacity_20c": 32.0, "refrigerant_type": "R600A", "external_dimensions": "87.2 x 112.8 x 71.7", "waterpack_storage_capacity": 16.0, "waterpack_freezing_capacity": 4.0, "energy_consumption_stable": 0.7, "energy_consumption_freezing": 0.97, "hold_over_time": 63.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/097';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 50.0, "refrigerant_type": "R600A", "external_dimensions": "158.8 x 54.5 x 65.5", "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/098';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 34.3, "refrigerant_type": "R600A", "external_dimensions": "85 x 55.5 x 65", "waterpack_storage_capacity": 17.4, "waterpack_freezing_capacity": 1.6, "energy_source": "Solar"}}' WHERE code = 'E003/099';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 36.5, "refrigerant_type": "R600A", "external_dimensions": "103 x 78 x 89", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.8, "hold_over_time": 121.9, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/100';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 80.5, "refrigerant_type": "R600A", "external_dimensions": "103 x 78 x 90", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.16, "hold_over_time": 72.15, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/101';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 75.0, "refrigerant_type": "R600A", "external_dimensions": "72 x 87.5 x 112.8", "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/102';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 58.0, "storage_capacity_20c": 44.0, "refrigerant_type": "R600A", "external_dimensions": "182 x 79.5 x 75", "waterpack_storage_capacity": 14.4, "waterpack_freezing_capacity": 2.4, "energy_consumption_stable": 1.91, "energy_consumption_freezing": 1.91, "hold_over_time": 113.62, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/103';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 55.5, "refrigerant_type": "R600A", "external_dimensions": "85 x 72 x 60", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "hold_over_time": 89.32, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/106';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 92.0, "refrigerant_type": "R600A", "external_dimensions": "86 x 93 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/107';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 170.0, "refrigerant_type": "R600A", "external_dimensions": "86 x 127 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "hold_over_time": 77.75, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/108';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 60.0, "refrigerant_type": "R600A", "external_dimensions": "85 x 73 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.57, "hold_over_time": 54.0, "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/109';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 98.0, "refrigerant_type": "R600A", "external_dimensions": "85 x 92 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.6, "hold_over_time": 55.5, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/110';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 127.0, "refrigerant_type": "R600A", "external_dimensions": "85 x 113 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.62, "hold_over_time": 54.7, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/111';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 145.0, "refrigerant_type": "R600A", "external_dimensions": "86 x 127 x 70", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.67, "hold_over_time": 55.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/112';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 242.0, "refrigerant_type": "R600A", "external_dimensions": "84.5 x 156.3 x 70", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 638.0, "hold_over_time": 55.27, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/113';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 100.0, "refrigerant_type": "R600A", "external_dimensions": "89 x 82.9 x 142.5", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.4, "hold_over_time": 128.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/114';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 200.0, "refrigerant_type": "R600A", "external_dimensions": "89 x 82.9 x 182", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.44, "hold_over_time": 87.23, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/115';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 100.0, "refrigerant_type": "R600A", "external_dimensions": "86.5 x 82.5 x 142.5", "waterpack_freezing_capacity": 0.0, "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/116';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 200.0, "refrigerant_type": "R600A", "external_dimensions": "86.5 x 82.5 x 181.5", "waterpack_freezing_capacity": 0.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/117';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 10.0, "external_dimensions": "52.8 x 86", "waterpack_freezing_capacity": 0.0, "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/118';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 61.25, "storage_capacity_20c": 34.3, "refrigerant_type": "R600A", "external_dimensions": "96 x 128 x 65", "waterpack_storage_capacity": 17.4, "waterpack_freezing_capacity": 1.8, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/119';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 38.0, "refrigerant_type": "R600A", "external_dimensions": "95 x 55 x 65", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 504.0, "hold_over_time": 57.52, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/120';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 80.5, "refrigerant_type": "R600A", "external_dimensions": "90 x 103 x 78", "waterpack_freezing_capacity": 0.0, "hold_over_time": 192.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/121';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 241.0, "refrigerant_type": "R290", "external_dimensions": "67 x 73 x 199.9", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 2.98, "hold_over_time": 25.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/122';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 120.0, "refrigerant_type": "R600A", "external_dimensions": "91 x 162 x 79", "waterpack_freezing_capacity": 1.6, "hold_over_time": 72.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/123';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 120.0, "storage_capacity_20c": 28.0, "refrigerant_type": "R600A", "external_dimensions": "91 x 162 x 79", "waterpack_freezing_capacity": 1.6, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/124';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_70c": 214.0, "refrigerant_type": "R290", "external_dimensions": "129.3 x 69.9 x 103.9", "energy_consumption_stable": 13.5, "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/125';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 61.0, "refrigerant_type": "R600A", "external_dimensions": "78.8 x 71.7 x 87.2", "energy_consumption_stable": 0.95, "hold_over_time": 7.32, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/126';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 68.0, "refrigerant_type": "R290", "external_dimensions": "71 x 72 x 95.5", "waterpack_storage_capacity": 72.5, "waterpack_freezing_capacity": 16.2, "energy_consumption_stable": 1.94, "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/127';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 166.0, "refrigerant_type": "R290", "external_dimensions": "123 x 72 x 96.5", "waterpack_storage_capacity": 203.0, "waterpack_freezing_capacity": 28.2, "energy_consumption_stable": 2.27, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/128';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 48.0, "storage_capacity_20c": 38.0, "refrigerant_type": "R290", "external_dimensions": "105 x 75 x 97.5", "waterpack_storage_capacity": 10.7, "waterpack_freezing_capacity": 2.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/129';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 153.0, "refrigerant_type": "R290", "external_dimensions": "76.2 x 82.5 x 85", "waterpack_storage_capacity": 130.8, "waterpack_freezing_capacity": 20.91, "energy_consumption_stable": 4.13, "hold_over_time": 9.82, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/130';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_20c": 211.0, "refrigerant_type": "R600A", "external_dimensions": "164.7 x 71.7 x 94", "energy_consumption_stable": 1.42, "energy_consumption_freezing": 1.4, "hold_over_time": 11.42, "climate_zone": "Hot", "energy_source": "Electricity"}}' WHERE code = 'E003/131';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 110.0, "storage_capacity_20c": 50.0, "refrigerant_type": "R600A", "external_dimensions": "87 x 151 x 74", "waterpack_storage_capacity": 17.4, "waterpack_freezing_capacity": 2.4, "hold_over_time": 114.33, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Solar"}}' WHERE code = 'E003/132';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 192.0, "refrigerant_type": "R290", "external_dimensions": "132 x 80.5 x 97", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.5, "climate_zone": "Hot, Temperate, Cold", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/133';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 1.55, "external_dimensions": "30 x 20 x 41", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 0.84, "hold_over_time": 12.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/134';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 100.0, "refrigerant_type": "R600A", "external_dimensions": "169.5 x 86.5 x 82.5", "waterpack_storage_capacity": 22.4, "waterpack_freezing_capacity": 2.0, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/135';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 226.4, "refrigerant_type": "R290", "external_dimensions": "126 x 84.6 x 84.7", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.67, "hold_over_time": 34.99, "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/136';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 103.5, "refrigerant_type": "R290", "external_dimensions": "76.2 x 84.6 x 84.7", "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.64, "hold_over_time": 25.95, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/137';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 240.0, "refrigerant_type": "R290", "external_dimensions": "160 x 78 x 91.5", "waterpack_storage_capacity": 252.0, "waterpack_freezing_capacity": 24.0, "energy_consumption_stable": 4.45, "energy_consumption_freezing": 4.45, "hold_over_time": 50.15, "climate_zone": "Hot", "freeze_protection": "Not tested", "energy_source": "Electricity"}}' WHERE code = 'E003/138';
UPDATE asset_catalogue_item SET properties = '{{"storage_capacity_5c": 164.5, "refrigerant_type": "R290", "external_dimensions": "10.1 x 84.2 x 84.5", "waterpack_storage_capacity": 0.0, "waterpack_freezing_capacity": 0.0, "energy_consumption_stable": 1.77, "hold_over_time": 40.22, "climate_zone": "Hot", "freeze_protection": "Grade A", "energy_source": "Electricity"}}' WHERE code = 'E003/139';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "36.2 x 28.3 x 29.9"}}' WHERE code = 'E004/002';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "32.6 x 10.7 x 20.2"}}' WHERE code = 'E004/003';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25.1 x 17.6 x 20.9"}}' WHERE code = 'E004/004';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "40.6 x 25.2 x 20.2"}}' WHERE code = 'E004/005';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "17.3 x 10.3 x 4.5"}}' WHERE code = 'E004/007';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "9 x 9.1 x 16.5"}}' WHERE code = 'E004/008';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "11.38 x 11.38 x 19"}}' WHERE code = 'E004/009';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "76.1 x 61.1 x 51.3"}}' WHERE code = 'E004/010';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25 x 18 x 12"}}' WHERE code = 'E004/011';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77.4 x 61.6 x 53"}}' WHERE code = 'E004/013';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77 x 61 x 51"}}' WHERE code = 'E004/015';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "39.9 x 39.6 x 14.5"}}' WHERE code = 'E004/017';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "61 x 60 x 56"}}' WHERE code = 'E004/018';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "49 x 44 x 39.5"}}' WHERE code = 'E004/019';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "26 x 26 32"}}' WHERE code = 'E004/020';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "26 x 25 x 28.5"}}' WHERE code = 'E004/021';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "2.5 x 16 x 25"}}' WHERE code = 'E004/022';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "62.4 x 50.2 x 42.6"}}' WHERE code = 'E004/023';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77 x 61.8 x 51.3"}}' WHERE code = 'E004/024';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "79.5 x 58.2 x 56.5"}}' WHERE code = 'E004/025';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "65 x 65 x 37"}}' WHERE code = 'E004/026';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "65 x 65 x 37"}}' WHERE code = 'E004/027';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "24.6 x 18 x 21.5"}}' WHERE code = 'E004/028';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "28 x 28 x 31.5"}}' WHERE code = 'E004/029';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "54.4 x 44.5 x 42"}}' WHERE code = 'E004/030';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "76.5 x 61.2 x 51.5"}}' WHERE code = 'E004/031';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "29 x 24 x 32"}}' WHERE code = 'E004/032';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "65 x 53 x 46"}}' WHERE code = 'E004/034';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77 x 62 x 53.5"}}' WHERE code = 'E004/036';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "27 x 27 x 32"}}' WHERE code = 'E004/040';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "52.8 x 74.7"}}' WHERE code = 'E004/041';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "50 x 37 x 38"}}' WHERE code = 'E004/042';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "28.8 x 28.9 x 33.7"}}' WHERE code = 'E004/043';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25 x 25 x 30"}}' WHERE code = 'E004/044';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "54 x 44.5 x 41.5"}}' WHERE code = 'E004/045';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77 x 61.5 x 51.5"}}' WHERE code = 'E004/046';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "29 x 29 x 32.7"}}' WHERE code = 'E004/047';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25.2 x 25.2 x 30.5"}}' WHERE code = 'E004/049';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "31 x 31 x 30"}}' WHERE code = 'E004/050';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "30.8 x 30.8 x 30"}}' WHERE code = 'E004/051';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "28.5 28.5 x 33.5"}}' WHERE code = 'E004/052';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25.3 x 25.3 x 30.5"}}' WHERE code = 'E004/053';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "25.2 x 26 x 30.5"}}' WHERE code = 'E004/054';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "29.5 x 29.5 x 33.5"}}' WHERE code = 'E004/055';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "49.3 x 45.5 x 39.7"}}' WHERE code = 'E004/056';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77 x 54 x 47"}}' WHERE code = 'E004/057';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "31.8 x 31.8 x 29.5"}}' WHERE code = 'E004/058';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "34.7 x 28.1 x 43"}}' WHERE code = 'E004/059';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "24.8 x 29 x 24.6"}}' WHERE code = 'E004/060';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "24.6 x 30.5 x 24.7"}}' WHERE code = 'E004/061';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "28.5 x 28.5 x 27"}}' WHERE code = 'E004/063';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77.5 x 54.5 x 47.3"}}' WHERE code = 'E004/064';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "32.5 x 32.5 x 32.7"}}' WHERE code = 'E004/065';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "31 x 31 x 30.5"}}' WHERE code = 'E004/066';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "78.5 x 63.2 x 53.3"}}' WHERE code = 'E004/067';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "78.5 x 63.2 x 53.3"}}' WHERE code = 'E004/068';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "77.5 x 54.5 x 48.5"}}' WHERE code = 'E004/069';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "22 x 38.7"}}' WHERE code = 'E004/070';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "28.5 x 30.5 x 29.5"}}' WHERE code = 'E004/071';
UPDATE asset_catalogue_item SET properties = '{{"external_dimensions": "29 x 28.5 x 27.8"}}' WHERE code = 'E004/072';
        "#,
    )?;

    Ok(())
}

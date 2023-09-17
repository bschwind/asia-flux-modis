use crate::data::DatasetMetadata;

pub fn get_dataset_names() -> Vec<String> {
    vec![
        "Lai".to_string(),
        "Fpar".to_string(),
        "EVI".to_string(),
        "NDVI".to_string(),
        "LST_Day".to_string(),
        "LST_Night".to_string(),
        "Nadir_Reflectance_Band1".to_string(),
        "Nadir_Reflectance_Band2".to_string(),
        "Nadir_Reflectance_Band3".to_string(),
        "Nadir_Reflectance_Band4".to_string(),
        "Nadir_Reflectance_Band5".to_string(),
        "Nadir_Reflectance_Band6".to_string(),
        "Nadir_Reflectance_Band7".to_string(),
    ]
}

pub fn get_dataset_metadata(dataset_name: &str) -> DatasetMetadata {
    let mut dataset_metadata = DatasetMetadata::default();
    if ["Lai", "Fpar"].contains(&dataset_name) {
        dataset_metadata = DatasetMetadata {
            dataset: dataset_name.to_string(),
            product: "MOD15A2H".to_string(),
            qc_name: "FparLai_QC".to_string(),
            data_bytes: 1,
            qc_bytes: 1,
            modis_size: String::from("500m"),
            data_type: String::from("u8"),
            qc_type: String::from("u8"),
            scale_factor: if dataset_name == "Lai" { 0.1 } else { 0.01 },
        }
    } else if ["NDVI", "EVI"].contains(&dataset_name) {
        dataset_metadata = DatasetMetadata {
            dataset: dataset_name.to_string(),
            product: "MOD13A2".to_string(),
            qc_name: "VI_Quality".to_string(),
            data_bytes: 2,
            qc_bytes: 2,
            modis_size: String::from("1km"),
            data_type: String::from("i16"),
            qc_type: String::from("u16"),
            scale_factor: 0.0001,
        }
    } else if ["LST_Day", "LST_Night"].contains(&dataset_name) {
        dataset_metadata = DatasetMetadata {
            dataset: dataset_name.to_string(),
            product: "MOD11A2".to_string(),
            qc_name: if dataset_name == "LST_Day" {
                "QC_Day".to_string()
            } else {
                "QC_Night".to_string()
            },
            data_bytes: 2,
            qc_bytes: 1,
            modis_size: String::from("1km"),
            data_type: String::from("u16"),
            qc_type: String::from("u8"),
            scale_factor: 0.02,
        }
    } else if dataset_name.contains("Nadir") {
        let band: String = dataset_name.chars().rev().take(1).collect();
        dataset_metadata = DatasetMetadata {
            dataset: dataset_name.to_string(),
            product: "MCD43A4".to_string(),
            qc_name: format!("BRDF_Albedo_Band_Mandatory_Quality_Band{}", band),
            data_bytes: 2,
            qc_bytes: 1,
            modis_size: String::from("500m"),
            data_type: String::from("i16"),
            qc_type: String::from("u8"),
            scale_factor: 0.0001,
        }
    }
    dataset_metadata
}

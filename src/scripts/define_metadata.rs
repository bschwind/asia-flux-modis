use crate::data::DatasetMetadata;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Dataset {
    Lai,
    Fpar,
    Evi,
    Ndvi,
    LstDay,
    LstNight,
    NadirReflectanceBand1,
    NadirReflectanceBand2,
    NadirReflectanceBand3,
    NadirReflectanceBand4,
    NadirReflectanceBand5,
    NadirReflectanceBand6,
    NadirReflectanceBand7,
}

impl Dataset {
    pub fn name(&self) -> String {
        let name = match self {
            Dataset::Lai => "Lai",
            Dataset::Fpar => "Fpar",
            Dataset::Evi => "EVI",
            Dataset::Ndvi => "NDVI",
            Dataset::LstDay => "LST_Day",
            Dataset::LstNight => "LST_Night",
            Dataset::NadirReflectanceBand1 => "Nadir_Reflectance_Band1",
            Dataset::NadirReflectanceBand2 => "Nadir_Reflectance_Band2",
            Dataset::NadirReflectanceBand3 => "Nadir_Reflectance_Band3",
            Dataset::NadirReflectanceBand4 => "Nadir_Reflectance_Band4",
            Dataset::NadirReflectanceBand5 => "Nadir_Reflectance_Band5",
            Dataset::NadirReflectanceBand6 => "Nadir_Reflectance_Band6",
            Dataset::NadirReflectanceBand7 => "Nadir_Reflectance_Band7",
        };

        name.to_string()
    }

    fn band(&self) -> Option<u8> {
        match self {
            Dataset::NadirReflectanceBand1 => Some(1),
            Dataset::NadirReflectanceBand2 => Some(2),
            Dataset::NadirReflectanceBand3 => Some(3),
            Dataset::NadirReflectanceBand4 => Some(4),
            Dataset::NadirReflectanceBand5 => Some(5),
            Dataset::NadirReflectanceBand6 => Some(6),
            Dataset::NadirReflectanceBand7 => Some(7),
            _ => None,
        }
    }
}

pub fn get_dataset_metadata(dataset: Dataset) -> DatasetMetadata {
    match dataset {
        Dataset::Lai | Dataset::Fpar => DatasetMetadata {
            dataset,
            product: "MOD15A2H".to_string(),
            qc_name: "FparLai_QC".to_string(),
            data_bytes: 1,
            qc_bytes: 1,
            modis_size: String::from("500m"),
            data_type: String::from("u8"),
            qc_type: String::from("u8"),
            scale_factor: if dataset == Dataset::Lai { 0.1 } else { 0.01 },
        },
        Dataset::Evi | Dataset::Ndvi => DatasetMetadata {
            dataset,
            product: "MOD13A2".to_string(),
            qc_name: "VI_Quality".to_string(),
            data_bytes: 2,
            qc_bytes: 2,
            modis_size: String::from("1km"),
            data_type: String::from("i16"),
            qc_type: String::from("u16"),
            scale_factor: 0.0001,
        },
        Dataset::LstDay | Dataset::LstNight => DatasetMetadata {
            dataset,
            product: "MOD11A2".to_string(),
            qc_name: if dataset == Dataset::LstDay {
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
        },
        Dataset::NadirReflectanceBand1
        | Dataset::NadirReflectanceBand2
        | Dataset::NadirReflectanceBand3
        | Dataset::NadirReflectanceBand4
        | Dataset::NadirReflectanceBand5
        | Dataset::NadirReflectanceBand6
        | Dataset::NadirReflectanceBand7 => {
            let band = dataset
                .band()
                .expect("NadirReflectance variants should have an associated band");

            DatasetMetadata {
                dataset,
                product: "MCD43A4".to_string(),
                qc_name: format!("BRDF_Albedo_Band_Mandatory_Quality_Band{band}"),
                data_bytes: 2,
                qc_bytes: 1,
                modis_size: String::from("500m"),
                data_type: String::from("i16"),
                qc_type: String::from("u8"),
                scale_factor: 0.0001,
            }
        }
    }
}

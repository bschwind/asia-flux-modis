use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TowerEntryData {
    pub year: i32,
    pub doy: u32,
    pub lat: f64,
    pub lon: f64,
}

pub struct DatasetMetadata {
    pub dataset: String,
    pub product: String,
    pub qc_name: String,
    pub data_bytes: u64,
    pub qc_bytes: u64,
    pub modis_size: String,
    pub data_type: String,
    pub qc_type: String,
    pub scale_factor: f64,
}

impl Default for DatasetMetadata {
    fn default() -> DatasetMetadata {
        DatasetMetadata {
            dataset: String::new(),
            product: String::new(),
            qc_name: String::new(),
            data_bytes: 0,
            qc_bytes: 0,
            modis_size: String::new(),
            data_type: String::new(),
            qc_type: String::new(),
            scale_factor: 0.0,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Test {
    pub id: u16,
}

// ideally I think these should all be some sort of option, I just got lazy.
#[derive(Default, Deserialize, Serialize)]
pub struct NewRecord<'a> {
    pub site_code: &'a str,
    pub lat: String,
    pub lon: String,
    pub syear: String,
    pub eyear: String,
    pub year: i32,
    pub doy: u32,
    pub solar_radiation: String,
    pub air_temperature: String,
    pub vpd: String,
    pub sensible_heat: String,
    pub evapotranspiration: String,
    pub respiration: String,
    pub nee: String,
    pub gpp: String,
    pub lai: Option<f64>,
    pub lai_goodpix: Option<f32>,
    pub fpar: Option<f64>,
    pub fpar_goodpix: Option<f32>,
    pub evi: Option<f64>,
    pub evi_goodpix: Option<f32>,
    pub ndvi: Option<f64>,
    pub ndvi_goodpix: Option<f32>,
    pub lst_day: Option<f64>,
    pub lst_day_goodpix: Option<f32>,
    pub lst_night: Option<f64>,
    pub lst_night_goodpix: Option<f32>,
    pub nadir_ref_band1: Option<f64>,
    pub nadir_ref_band1_goodpix: Option<f32>,
    pub nadir_ref_band2: Option<f64>,
    pub nadir_ref_band2_goodpix: Option<f32>,
    pub nadir_ref_band3: Option<f64>,
    pub nadir_ref_band3_goodpix: Option<f32>,
    pub nadir_ref_band4: Option<f64>,
    pub nadir_ref_band4_goodpix: Option<f32>,
    pub nadir_ref_band5: Option<f64>,
    pub nadir_ref_band5_goodpix: Option<f32>,
    pub nadir_ref_band6: Option<f64>,
    pub nadir_ref_band6_goodpix: Option<f32>,
    pub nadir_ref_band7: Option<f64>,
    pub nadir_ref_band7_goodpix: Option<f32>,
}

// pub trait CreateHeader {
//     fn create_header() -> NewRecord;
// }

// impl CreateHeader for NewRecord {
//     fn create_header() -> NewRecord {
//         NewRecord {
//             site_code: "SiteCode".to_string(),
//             lat: "LAT".to_string(),
//             lon: "LON".to_string(),
//             syear: "SYEAR".to_string(),
//             eyear: "EYEAR".to_string(),
//             year: "Year".to_string(),
//             doy: "DOY".to_string(),
//             solar_radiation: "SolarRadiation".to_string(),
//             air_temperature: "AirTemperature".to_string(),
//             vpd: "VPD".to_string(),
//             sensible_heat: "SensibleHeat".to_string(),
//             evapotranspiration: "Evapotranspiration".to_string(),
//             respiration: "Respiration".to_string(),
//             nee: "NEE".to_string(),
//             gpp: "GPP".to_string(),
//             lai: "Lai".to_string(),
//             lai_goodpix: "per_goodpix".to_string(),
//             fpar: "Fpar".to_string(),
//             fpar_goodpix: "per_goodpix".to_string(),
//             evi: "EVI".to_string(),
//             evi_goodpix: "per_goodpix".to_string(),
//             ndvi: "NDVI".to_string(),
//             ndvi_goodpix: "per_goodpix".to_string(),
//             lst_day: "LST_Day".to_string(),
//             lst_day_goodpix: "per_goodpix".to_string(),
//             lst_night: "LST_Night".to_string(),
//             lst_night_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band1: "Nadir_Reflectance_Band1".to_string(),
//             nadir_ref_band1_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band2: "Nadir_Reflectance_Band2".to_string(),
//             nadir_ref_band2_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band3: "Nadir_Reflectance_Band3".to_string(),
//             nadir_ref_band3_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band4: "Nadir_Reflectance_Band4".to_string(),
//             nadir_ref_band4_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band5: "Nadir_Reflectance_Band5".to_string(),
//             nadir_ref_band5_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band6: "Nadir_Reflectance_Band6".to_string(),
//             nadir_ref_band6_goodpix: "per_goodpix".to_string(),
//             nadir_ref_band7: "Nadir_Reflectance_Band7".to_string(),
//             nadir_ref_band7_goodpix: "per_goodpix".to_string(),
//         }
//     }
// }

// pub trait Vectorize {
//     fn vectorize(&self) -> Vec<&str>;
// }

// impl Vectorize for NewRecord {
//     fn vectorize(&self) -> Vec<&str> {
//         vec![
//             self.site_code.as_str(),
//             self.lat.as_str(),
//             self.lon.as_str(),
//             self.syear.as_str(),
//             self.eyear.as_str(),
//             self.year.as_str(),
//             self.doy.as_str(),
//             self.solar_radiation.as_str(),
//             self.air_temperature.as_str(),
//             self.vpd.as_str(),
//             self.sensible_heat.as_str(),
//             self.evapotranspiration.as_str(),
//             self.respiration.as_str(),
//             self.nee.as_str(),
//             self.gpp.as_str(),
//             self.lai.as_str(),
//             self.lai_goodpix.as_str(),
//             self.fpar.as_str(),
//             self.fpar_goodpix.as_str(),
//             self.evi.as_str(),
//             self.evi_goodpix.as_str(),
//             self.ndvi.as_str(),
//             self.ndvi_goodpix.as_str(),
//             self.lst_day.as_str(),
//             self.lst_day_goodpix.as_str(),
//             self.lst_night.as_str(),
//             self.lst_night_goodpix.as_str(),
//             self.nadir_ref_band1.as_str(),
//             self.nadir_ref_band1_goodpix.as_str(),
//             self.nadir_ref_band2.as_str(),
//             self.nadir_ref_band2_goodpix.as_str(),
//             self.nadir_ref_band3.as_str(),
//             self.nadir_ref_band3_goodpix.as_str(),
//             self.nadir_ref_band4.as_str(),
//             self.nadir_ref_band4_goodpix.as_str(),
//             self.nadir_ref_band5.as_str(),
//             self.nadir_ref_band5_goodpix.as_str(),
//             self.nadir_ref_band6.as_str(),
//             self.nadir_ref_band6_goodpix.as_str(),
//             self.nadir_ref_band7.as_str(),
//             self.nadir_ref_band7_goodpix.as_str(),
//         ]
//     }
// }

use crate::data::*;
use crate::scripts::define_metadata::*;
use crate::scripts::get_modis_data::get_modis_data;
use chrono::prelude::*;
use csv::{Position, ReaderBuilder, WriterBuilder};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    fs::{self, remove_file},
    path::{Path, PathBuf},
    str,
    time::Instant,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    // MODIS is the name of the nasa sensor used to collect the binary data I'm using
    let modis_dir = "/modis/ORG/binary_data";
    let output_dir = "./output";
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // A site code is an id for a tower. I'm getting the locations of the towers
    let mut site_codes: Vec<String> = Vec::new();

    // Get list of site names
    for result in rdr.records() {
        let record = result?;
        let site_code = record.get(3).unwrap();
        site_codes.push(site_code.to_owned());
    }

    // convert into &str for better performance.
    // negligible effect compared to reading the binary data
    let mut site_codes_str = site_codes
        .iter()
        .map(|code| code.as_str())
        .collect::<Vec<&str>>();

    // Get only unique site names
    site_codes_str.sort();
    site_codes_str.dedup();

    // create output directory if it doesn't exist
    if !Path::new(output_dir).is_dir() {
        fs::create_dir(output_dir)?;
    }

    for (i, site_code) in site_codes_str.iter().enumerate() {
        // site_codes_str includes header "SiteCode", so skip that one
        if site_code == &"SiteCode" {
            continue;
        }

        // measures time for each site iteration
        let instant = Instant::now();
        println!("SITE {i}/{}: {}", site_codes_str.len(), site_code);

        // file for new csv. each site gets its own file
        let site_file_str = format!("{}/{}.csv", output_dir, site_code);
        let site_file_path = Path::new(&site_file_str);

        // skip previusly processed sites
        if site_file_path.exists() {
            // remove_file(site_file_path)?;
            continue;
        }

        // initialize csv writer
        let mut wtr = WriterBuilder::new()
            .flexible(false)
            .from_path(site_file_path)?;

        // NewRecord struct. Each field represents on column of new csv file.
        // Represents line which will be written to new csv.
        let mut rcrd = NewRecord::default();
        rcrd.site_code = site_code;

        // Start at beginning of file for each site code
        rdr.seek(Position::new())?;

        //get position(line) of input csv where site data starts (it's sorted by site and then time, all sites are in one file)
        let mut results = rdr.records();
        let pos: Position;
        loop {
            let next_pos = results.reader().position().clone();
            if &results.next().unwrap()?.get(3).unwrap() == site_code {
                pos = next_pos;
                break;
            }
        }

        // Start at beginning of file for each site code
        rdr.seek(Position::new())?;

        // Get unchanging column values from site (latitiude, longitute, start year, end year)
        for result in rdr.records() {
            let record = result?;
            if &record.get(3).unwrap() == site_code {
                rcrd.lat = record.get(5).unwrap().to_string();
                rcrd.lon = record.get(6).unwrap().to_string();
                rcrd.syear = record.get(8).unwrap().to_string();
                rcrd.eyear = record.get(9).unwrap().to_string();
                //stop iterating as they do not change over time
                break;
            }
        }

        // go to csv line where site data begins
        rdr.seek(pos)?;

        // Each new csv file will start at 2000 and go through 2020.
        for year in 2000..=2020 {
            // add year to new csv record
            rcrd.year = year;
            // every 8 days
            for doy in (1..=361).step_by(8) {
                println!("{year}.{doy}");
                // add doy to new csv record
                rcrd.doy = doy;
                // once year reaches the year that the input csv already has data for, write that data to the new record
                if year >= rcrd.syear.parse::<i32>().unwrap()
                    && year <= rcrd.eyear.parse::<i32>().unwrap()
                {
                    let record = rdr.records().next().unwrap()?;
                    rcrd.solar_radiation = record.get(13).unwrap().to_string();
                    rcrd.air_temperature = record.get(14).unwrap().to_string();
                    rcrd.vpd = record.get(15).unwrap().to_string();
                    rcrd.sensible_heat = record.get(16).unwrap().to_string();
                    rcrd.evapotranspiration = record.get(17).unwrap().to_string();
                    rcrd.respiration = record.get(18).unwrap().to_string();
                    rcrd.nee = record.get(19).unwrap().to_string();
                    rcrd.gpp = record.get(20).unwrap().to_string();
                } else {
                    // years that fall outside of start or end year will have empty data for these columns
                    rcrd.solar_radiation = "".to_string();
                    rcrd.air_temperature = "".to_string();
                    rcrd.vpd = "".to_string();
                    rcrd.sensible_heat = "".to_string();
                    rcrd.evapotranspiration = "".to_string();
                    rcrd.respiration = "".to_string();
                    rcrd.nee = "".to_string();
                    rcrd.gpp = "".to_string();
                }
                // a dataset name represents one set of binary files. Each one measure something different (temp/vegetation level/etc)
                for dataset_name in get_dataset_names() {
                    // get descriptive data about the dataset binary files
                    let dm = get_dataset_metadata(&dataset_name);
                    // check if the corresponding dataset file exists for the corresponding date, return the data file path and the quality control file path
                    // both are binary files. QC just shows if a pixel in that location is reliable or not. The data file has the actual measured value. I have to read both.
                    let data_qc_paths = check_if_modis_data_exists(year, doy, modis_dir, &dm);
                    match data_qc_paths {
                        Ok(data_qc_paths) => {
                            // GET THE MODIS DATA FROM BINARY FILES
                            // WRITE DATA TO rcrd
                            rcrd = get_modis_data(rcrd, dm, data_qc_paths);
                        }
                        Err(_e) => {
                            // println!("{dataset_name}: {e}")
                        }
                    }
                }
                // Write record to csv file.
                match wtr.serialize(&rcrd) {
                    Ok(_res) => {}
                    Err(error) => panic!("Problem writing record: {error}"),
                }
                wtr.flush()?;
            }
        }
        println!("{:?}", instant.elapsed().as_secs() / 60);
    }
    Ok(())
}

fn check_if_modis_data_exists(
    year: i32,
    doy: u32,
    modis_dir: &str,
    dm: &DatasetMetadata,
) -> Result<(PathBuf, PathBuf), &'static str> {
    let naive_date = NaiveDate::from_yo_opt(year, doy).unwrap();
    let date = naive_date.format("%Y.%m.%d");

    let mut data_file_string = String::new();
    let mut qc_file_string = String::new();

    if ["NDVI", "EVI"].contains(&dm.dataset.as_str()) {
        data_file_string = format!(
            "{}.061.{}.1_km_16_days_{}.bsq",
            dm.product,
            date,
            dm.dataset.as_str()
        );
        qc_file_string = format!("{}.061.{}.1_km_16_days_VI_Quality.bsq", dm.product, date);
    } else if ["Lai", "Fpar", "LST_Day", "LST_Night"].contains(&dm.dataset.as_str()) {
        data_file_string = format!(
            "{}.061.{}.{}_{}.bsq",
            dm.product, date, dm.dataset, dm.modis_size
        );
        qc_file_string = format!("{}.061.{}.{}.bsq", dm.product, date, dm.qc_name);
    } else if dm.dataset.as_str().contains("Nadir") {
        data_file_string = format!("{}.061.{}.{}.bsq", dm.product, date, dm.dataset);
        qc_file_string = format!("{}.061.{}.{}.bsq", dm.product, date, dm.qc_name);
    }

    let file_path_string = format!(
        "{}/{}.061/{}_org/{}",
        modis_dir, dm.product, dm.modis_size, data_file_string
    );
    let qc_file_path_string = format!(
        "{}/{}.061/{}_org/{}",
        modis_dir, dm.product, dm.modis_size, qc_file_string,
    );
    let file_path = PathBuf::from(&file_path_string);
    let qc_file_path = PathBuf::from(&qc_file_path_string);
    let paths_exist = (file_path.exists(), qc_file_path.exists());

    match paths_exist {
        (true, true) => {
            let data_qc_paths = (file_path, qc_file_path);
            Ok(data_qc_paths)
        }
        (true, false) => Err("No QC file found"),
        (false, true) => Err("No data file found"),
        (false, false) => Err("No data file or QC file found"),
    }
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected fluxnet data file path, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

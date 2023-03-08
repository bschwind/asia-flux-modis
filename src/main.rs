use chrono::prelude::*;
use csv::StringRecord;
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io,
    io::{BufReader, Read, Seek, SeekFrom},
    process,
};

#[derive(Debug, Clone, Default)]
struct ModisBin500m {
    first: [u8; 7],
    second: [u8; 7],
    third: [u8; 7],
    fourth: [u8; 7],
    fifth: [u8; 7],
    sixth: [u8; 7],
    seventh: [u8; 7],
}

#[derive(Debug)]
struct TowerEntryData {
    year: i32,
    doy: u32,
    site_code: String,
    lat: f64,
    lon: f64,
}

const PIXELS: u64 = 86400;
const LINES: u64 = 43200;
const PIXEL_SIZE: f64 = 180.0 / LINES as f64;

fn get_modis_data(record: StringRecord) -> io::Result<()> {
    let mut modis_binary: ModisBin500m = Default::default();

    let tower_entry_data = TowerEntryData {
        year: record.get(11).unwrap().parse().unwrap(),
        doy: record.get(12).unwrap().parse().unwrap(),
        site_code: record.get(3).unwrap().parse().unwrap(),
        lat: record.get(5).unwrap().parse().unwrap(),
        lon: record.get(6).unwrap().parse().unwrap(),
    };

    if tower_entry_data.site_code == "YPF" {
        let date = NaiveDate::from_yo_opt(tower_entry_data.year, tower_entry_data.doy).unwrap();
        let date_format = date.format("%Y.%m.%d");

        let file_string = format!(
            "/modis01/dan/data/MOD15A2H.061/500m_org/MOD15A2H.061.{}.Lai_500m.bsq",
            date_format,
        );

        println!("{file_string}");
        let seek_line: u64 = LINES - ((tower_entry_data.lat + 90.0) / PIXEL_SIZE) as u64 - 1;
        let seek_pixel: u64 = ((tower_entry_data.lon + 180.0) / PIXEL_SIZE) as u64;
        let seek_point: u64 = (seek_line * PIXELS - 1) + seek_pixel;
        let mut file = BufReader::new(File::open(file_string)?);

        file.seek(SeekFrom::Start(seek_point - 3 - (PIXELS * 3)))?;
        file.read_exact(&mut modis_binary.first)?;
        file.seek(SeekFrom::Start(seek_point - 3 - (PIXELS * 2)))?;
        file.read_exact(&mut modis_binary.second)?;
        file.seek(SeekFrom::Start(seek_point - 3 - PIXELS))?;
        file.read_exact(&mut modis_binary.third)?;
        file.seek(SeekFrom::Start(seek_point - 3))?;
        file.read_exact(&mut modis_binary.fourth)?;

        let mut value: [u8; 1] = Default::default();
        file.seek(SeekFrom::Start(seek_point))?;
        file.read_exact(&mut value)?;
        println!("seek_line: {seek_line}");
        println!("seek_pixel: {seek_pixel}");
        println!("exact value: {value:?}");
        println!("tower data: {:?}", tower_entry_data);
        println!("value: {:?}", modis_binary);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        get_modis_data(record);
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

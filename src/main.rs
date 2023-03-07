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
const PIXEL_SIZE: f64 = (180 / LINES) as f64;

fn get_modis_data(record: StringRecord) -> io::Result<()> {
    let mut modis_binary: ModisBin500m = Default::default();

    let tower_entry_data = TowerEntryData {
        year: record.get(11).unwrap().parse().unwrap(),
        doy: record.get(12).unwrap().parse().unwrap(),
        site_code: record.get(3).unwrap().parse().unwrap(),
        lat: record.get(5).unwrap().parse().unwrap(),
        lon: record.get(6).unwrap().parse().unwrap(),
    };

    let date = NaiveDate::from_yo_opt(tower_entry_data.year, tower_entry_data.doy).unwrap();
    let date_format = date.format("%Y.%m.%d");

    let file_string = format!(
        "/modis01/dan/data/MOD15A2H.061/500m_org/MOD15A2H.061.{}.Lai_500m.bsq",
        date_format,
    );

    println!("{file_string}");
    let seek_line = (tower_entry_data.lat + 90.0) / PIXEL_SIZE;
    let seek_pixel = (tower_entry_data.lon + 180.0) / PIXEL_SIZE;
    let seek_point = (seek_line.round() as u64 * PIXELS - 1) + seek_pixel.round() as u64;
    let mut file = BufReader::new(File::open(file_string)?);
    file.seek(SeekFrom::Start(seek_point - 3))?;
    file.read_exact(&mut modis_binary.fourth)?;
    println!("value: {:?}", modis_binary.fourth);

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

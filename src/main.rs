use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io,
    io::{BufReader, Read, Seek, SeekFrom},
    process,
};

use csv::StringRecord;

#[derive(Debug, Clone, Default)]
struct ModisBinary {
    values: [u8; 1],
}

fn get_modis_data(record: StringRecord) -> io::Result<()> {
    let mut modis_binary: ModisBinary = Default::default();
    let lat = record.get(5);
    let lon = record.get(6);

    let mut file = BufReader::new(File::open("AsiaDB_C6.csv")?);
    // let mut file = File::open("AsiaDB_C6.csv")?;
    // let file2 = file.into_inner();
    file.seek(SeekFrom::Start(1001))?;
    file.read_exact(&mut modis_binary.values)?;
    println!("{:?}", modis_binary.values);

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        get_modis_data(record);
        // println!("{:?}", record);
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

use bytemuck::cast_slice;
// use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::data::*;

fn find_mesh_values<const DW: usize, const DH: usize, const QCW: usize, const QCH: usize>(
    dm: &DatasetMetadata,
    tower_entry_data: &TowerEntryData,
    data_qc_paths: (PathBuf, PathBuf),
    // generic array lengths (data width, data height, quality control width, qc height)
    mut data_array: [[u8; DW]; DH],
    mut qc_array: [[u8; QCW]; QCH],
) -> (Option<f64>, Option<f32>) {
    // remote sensing jargon
    // think of the binary file as a 2d raster image
    // pixels is the number of pixels along the x axis
    // lines is the number of pixels along the y axis
    // this is the resolution of the most detailed files (500m pixels)
    let mut pixels = 86400;
    let mut lines = 43200;

    // if pixel size is 1km, divide these values by two to get an accurate measurement
    if dm.modis_size == "1km" {
        pixels = pixels / 2;
        lines = lines / 2;
    }
    // 1km and 500m are actually not accurate, each pixel actually represents a certain number of degrees squared on earth
    // this finds that number.
    // this doesn't really matter but the actual earth area the pixels represent change depending on the latitude as the pixels are mapped to degrees.
    let pixel_size: f64 = 180.0 / lines as f64;

    // some calculations to find out which pixel the tower is located in for the data file and quality control file
    // out put is seek_point. which is the pixel minus half of the length of the width of the area we want to read (rounded up??? cant remember doesnt really matter)
    // so we can for example read 3 u8 pixels from left to right, with the tower being the middle pixel.
    let data_array_len = data_array.len() as u64;
    let data_matrix_width = data_array[0].len() as i64;
    let qc_array_len = qc_array.len() as u64;
    let qc_matrix_width = qc_array[0].len() as i64;

    let seek_line: u64 = lines - ((tower_entry_data.lat + 90.0) / pixel_size) as u64;
    let seek_pixel: u64 = ((tower_entry_data.lon + 180.0) / pixel_size) as u64;
    let data_seek_point: u64 =
        (((seek_line - 1) * pixels) + (seek_pixel - (data_array_len / 2))) * dm.data_bytes;
    let qc_seek_point: u64 =
        (((seek_line - 1) * pixels) + (seek_pixel - (qc_array_len / 2))) * dm.qc_bytes;

    // define files to read
    let mut data_file = BufReader::new(File::open(data_qc_paths.0).unwrap());
    let mut qc_file = BufReader::new(File::open(data_qc_paths.1).unwrap());

    //read data into arrays
    // if the matrix we are reading to is 7x7, the seek point is in the middle row
    // so first we subtract the total length of the file in its relevant data type * 3, which gets us to the top left pixel,
    // read that row, then .seek to the next row by adding the length of the file and continue.
    // this uses bufreader seek. I'm 99% sure this takes 99.9% of the time.
    for (i, array) in data_array.iter_mut().enumerate() {
        let line_multiplier: i64 = i as i64 - data_array_len as i64 / 2;
        if i == 0 {
            data_file
                .seek(SeekFrom::Start(
                    (data_seek_point as i64
                        + (pixels as i64 * dm.data_bytes as i64 * line_multiplier))
                        as u64,
                ))
                .unwrap();
        } else {
            data_file
                .seek_relative((pixels as i64 * dm.data_bytes as i64) - data_matrix_width)
                .unwrap();
        }
        data_file.read_exact(array).unwrap();
    }

    // same for qc file
    for (i, array) in qc_array.iter_mut().enumerate() {
        let line_multiplier: i64 = i as i64 - qc_array_len as i64 / 2;
        if i == 0 {
            qc_file
                .seek(SeekFrom::Start(
                    (qc_seek_point as i64 + (pixels as i64 * dm.qc_bytes as i64 * line_multiplier))
                        as u64,
                ))
                .unwrap();
        } else {
            qc_file
                .seek_relative((pixels as i64 * dm.qc_bytes as i64) - qc_matrix_width)
                .unwrap();
        }
        qc_file.read_exact(array).unwrap();
    }

    // convert data and qc data to flat vec
    let flattened_qc_u8: Vec<u8> = qc_array.into_iter().flatten().collect();
    let flattened_data_u8: Vec<u8> = data_array.into_iter().flatten().collect();
    let mut flattened_data_f64: Vec<f64> = vec![];
    if dm.data_bytes == 2 {
        if dm.data_type == "i16" {
            let d: &[i16] = cast_slice(&flattened_data_u8);
            flattened_data_f64 = d.iter().map(|item| item.clone() as f64).collect();
        } else if dm.data_type == "u16" {
            let d: &[u16] = cast_slice(&flattened_data_u8);
            flattened_data_f64 = d.iter().map(|item| item.clone() as f64).collect();
        }
    } else if dm.data_bytes == 1 {
        flattened_data_f64 = flattened_data_u8
            .into_iter()
            .map(|item| item as f64)
            .collect();
    }

    // Get count of null values in data vecs and good quality data from qc vecs

    // each dataset is in a parent "product" from nasa. I just use the product here so I don't have to specify each dataset
    // as some datasets share a parent product and share a null value (-3000 for example)
    let data_len = flattened_data_f64.len() as f32;
    let mut null_val_count: f32 = 0.0;
    let mut good_qc_count = 0;
    if dm.product == "MOD15A2H" {
        null_val_count = flattened_data_f64.iter().filter(|&x| x >= &249.0).count() as f32;
        flattened_data_f64.retain_mut(|&mut x| x < 249.0);
        good_qc_count = flattened_qc_u8
            .iter()
            .filter(|x| x.trailing_zeros() >= 1)
            .count();
    } else if dm.product == "MOD13A2" {
        null_val_count = flattened_data_f64.iter().filter(|&x| x == &-3000.0).count() as f32;
        flattened_data_f64.retain_mut(|&mut x| x != -3000.0);
        let qc: &[u16] = cast_slice(&flattened_qc_u8);
        good_qc_count = qc.iter().filter(|x| x.trailing_zeros() >= 2).count();
    } else if dm.product == "MOD11A2" {
        null_val_count = flattened_data_f64.iter().filter(|&x| x == &0.0).count() as f32;
        flattened_data_f64.retain_mut(|&mut x| x != 0.0);
        good_qc_count = flattened_qc_u8
            .iter()
            .filter(|x| x.trailing_zeros() >= 2)
            .count();
    } else if dm.product == "MCD43A4" {
        null_val_count = flattened_data_f64.iter().filter(|&x| x == &32767.0).count() as f32;
        flattened_data_f64.retain_mut(|&mut x| x != 32767.0);
        good_qc_count = flattened_qc_u8
            .into_iter()
            .filter(|x| x.clone() == 0 as u8)
            .count();
    }

    // if nulls are more than half of data, return empty string
    // not enough data to justify using
    if (null_val_count / data_len as f32) > 0.5 {
        (None, None)
    } else {
        // Get goodpix percent
        let goodpix_per = good_qc_count as f32 / flattened_data_f64.len() as f32;

        // apply scale factors
        flattened_data_f64 = flattened_data_f64
            .into_iter()
            .map(|x| x * dm.scale_factor)
            .collect();
        let array_sum: f64 = flattened_data_f64.iter().sum();
        let array_mean = array_sum / flattened_data_f64.len() as f64;
        let array_mean_round = (array_mean * 10000.0).round() / 10000.0;

        // return data average, and good quality pixel percentage from relevant matrices. these values are put into the csv record.
        (Some(array_mean_round), Some(goodpix_per))
    }
}

pub fn get_modis_data(
    mut rcrd: NewRecord,
    dm: DatasetMetadata,
    data_qc_paths: (PathBuf, PathBuf),
) -> NewRecord {
    let tower_entry_data = TowerEntryData {
        year: rcrd.year,
        doy: rcrd.doy,
        lat: rcrd.lat.parse().unwrap(),
        lon: rcrd.lon.parse().unwrap(),
    };

    // Create nested arrays for each dataset. think of these a matrix. data types with 1km pixels get a 3x3km array
    // 500km pixels get a 7x7 array of 500m pixels, the tower coordinates are located in the center pixel of each.
    // some rows have double the length because their data type is 16 bit and I convert from u8 to 16 bit after I read it.
    let array_lai: [[u8; 7]; 7] = Default::default();
    let array_lai_qc: [[u8; 7]; 7] = Default::default();

    let array_fpar: [[u8; 7]; 7] = Default::default();
    let array_fpar_qc: [[u8; 7]; 7] = Default::default();

    // Arrays are double in size and u8, will convert to 3x3 i16 later
    let array_evi: [[u8; 6]; 3] = Default::default();
    let array_ndvi: [[u8; 6]; 3] = Default::default();
    let array_vi_qc: [[u8; 6]; 3] = Default::default();

    let array_lst_day: [[u8; 6]; 3] = Default::default();
    let array_lst_night: [[u8; 6]; 3] = Default::default();
    let array_lst_qc: [[u8; 3]; 3] = Default::default();

    let array_nrb: [[u8; 14]; 7] = Default::default();
    let array_brdf: [[u8; 7]; 7] = Default::default();

    let data_ave;
    let goodpix_per;

    // couldn't figure out any other way to do this than replicate it for every dataset lol.
    // calls above function find_mesh_value, which is when it reads the binary files.
    // it then writes the data it reads and processes to the new csv record
    if dm.dataset.as_str() == "Lai" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_lai,
            array_lai_qc,
        );
        rcrd.lai = data_ave;
        rcrd.lai_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str() == "Fpar" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_fpar,
            array_fpar_qc,
        );
        rcrd.fpar = data_ave;
        rcrd.fpar_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str() == "EVI" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_evi,
            array_vi_qc,
        );
        rcrd.evi = data_ave;
        rcrd.evi_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str() == "NDVI" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_ndvi,
            array_vi_qc,
        );
        rcrd.ndvi = data_ave;
        rcrd.ndvi_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str() == "LST_Day" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_lst_day,
            array_lst_qc,
        );
        rcrd.lst_day = data_ave;
        rcrd.lst_day_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str() == "LST_Night" {
        (data_ave, goodpix_per) = find_mesh_values(
            &dm,
            &tower_entry_data,
            data_qc_paths,
            array_lst_night,
            array_lst_qc,
        );
        rcrd.lst_night = data_ave;
        rcrd.lst_night_goodpix = goodpix_per;
        rcrd
    } else if dm.dataset.as_str().contains("Nadir") {
        (data_ave, goodpix_per) =
            find_mesh_values(&dm, &tower_entry_data, data_qc_paths, array_nrb, array_brdf);
        match dm.dataset.as_str() {
            "Nadir_Reflectance_Band1" => {
                rcrd.nadir_ref_band1 = data_ave;
                rcrd.nadir_ref_band1_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band2" => {
                rcrd.nadir_ref_band2 = data_ave;
                rcrd.nadir_ref_band2_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band3" => {
                rcrd.nadir_ref_band3 = data_ave;
                rcrd.nadir_ref_band3_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band4" => {
                rcrd.nadir_ref_band4 = data_ave;
                rcrd.nadir_ref_band4_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band5" => {
                rcrd.nadir_ref_band5 = data_ave;
                rcrd.nadir_ref_band5_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band6" => {
                rcrd.nadir_ref_band6 = data_ave;
                rcrd.nadir_ref_band6_goodpix = goodpix_per;
            }
            "Nadir_Reflectance_Band7" => {
                rcrd.nadir_ref_band7 = data_ave;
                rcrd.nadir_ref_band7_goodpix = goodpix_per;
            }
            _ => {}
        }
        rcrd
    } else {
        rcrd
    }

    // Ok(new_record?)
}

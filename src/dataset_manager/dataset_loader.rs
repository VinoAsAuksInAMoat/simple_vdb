use std::{
    fs::{File, metadata}, 
    fs, 
    io::{Read, BufReader}, 
    io, 
    time, 
    time::Duration, 
    rc::Rc, 
    collections::HashMap, 
    cmp, 
};

use crate::common::{
    data::datatypes::*, 
    data::neighbor::*, 
    data::search_result::*, 
};


pub trait Load {
    fn load(&self, filename: &str) -> Result<Dataset, io::Error> {
        //println!("[Info] Load file: {}", filename);
        let timer = time::Instant::now();
        let (data_dim, data_num) = match Self::get_data_dim_and_num(filename) {
            Ok((data_dim, data_num)) => (data_dim, data_num), 
            Err(error) => {
                panic!("There was a problem loading the dataset: {:?}", error)
            }, 
        };
        Self::sequentially_load(filename, data_dim, data_num)
        
    }
    fn partial_load(&self, filename: &str, load_data_num: u64) -> Result<Dataset, io::Error> {
        //println!("[Info] Load file: {}", filename);
        let timer = time::Instant::now();
        let (data_dim, data_num) = match Self::get_data_dim_and_num(filename) {
            Ok((data_dim, data_num)) => (data_dim, data_num), 
            Err(error) => {
                panic!("There was a problem loading the dataset: {:?}", error)
            }, 
        };
        Self::sequentially_load(filename, data_dim, std::cmp::min(data_num, load_data_num))

    }
    fn sequentially_load(filename: &str, data_dim: Dim, data_num: u64) -> Result<Dataset, io::Error>;
    fn get_data_dim_and_num(valid_filename: &str) -> Result<(Dim, u64), io::Error>;
}

pub struct Fvecs;

impl Load for Fvecs {
    fn sequentially_load(filename: &str, data_dim: Dim, data_num: u64) -> Result<Dataset, io::Error> {
        let mut reader = BufReader::new(File::open(filename)?);
        let mut buf: [u8; 4] = [0;4];

        let mut dataset = Dataset::new (data_dim);

        for _ in 0..data_num {
            let _ = reader.read(&mut buf);
            let dim = Dim::from_le_bytes(buf);
            if dataset.dim != dim {
                panic!("The dimensions of data are not same");
            }

            let mut row = Vec::new();
            for _j in 0..(dataset.dim) {
                let _ = reader.read(&mut buf);
                let val: f32 = f32::from_le_bytes(buf);
                row.push(val);
            }
            let _ = dataset.add(&row);
        }

        Ok(dataset)

    }


    fn get_data_dim_and_num(valid_filename: &str) -> Result<(Dim, u64), io::Error> {
        let metadata = fs::metadata(valid_filename)?;
        let mut reader = BufReader::new(File::open(valid_filename)?);

        let mut buf: [u8; 4] = [0;4];

        let _ = reader.read(&mut buf);
        let dim = Dim::from_le_bytes(buf);
        
        Ok((dim, metadata.len() as u64 / ((dim) as u64 * 4 + 4)))

    }
}
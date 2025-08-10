use std::fs::File;
use std::io::{Read, BufReader};
use std::time;

use crate::data;

pub fn load_fvecs(filename: String, data_num: u64) -> data::Data {
    println!("[Info] Load file: {}", filename);

    let timer = time::Instant::now();

    let mut reader = BufReader::new(File::open(filename).unwrap());

    let mut buf: [u8; 4] = [0;4];
    let mut data = Vec::new();
    let mut dim: u32 = 0;
    for _i in 0..data_num {
        let _ = reader.read(&mut buf);
        dim = u32::from_le_bytes(buf);

        let mut row = Vec::new();
        for _j in 0..dim {
            let _ = reader.read(&mut buf);
            let val: f32 = f32::from_le_bytes(buf);
            row.push(val);
        }
        data.push(row);
    }

    println!("[Info] -> completed: {:?}", timer.elapsed());
    
    data::Data {
        dim: dim, 
        num: data_num as u64, 
        data: data.clone(), 
    }

}


use std::fs::File;
use std::io::{Read, BufReader};
use std::{time, rc::Rc};
use std::collections::HashMap;

use crate::common::data::*;

pub fn load_fvecs(filename: &str, data_num: u64) -> Dataset {
    println!("[Info] Load file: {}", filename);

    let timer = time::Instant::now();

    let mut reader = BufReader::new(File::open(filename).unwrap());

    let mut dataset = Dataset {
        dim: 0, 
        num: data_num, 
        data: HashMap::new(), 
    };

    let mut buf: [u8; 4] = [0;4];
    let mut dim: Dim = 0;
    for id in 0..data_num {
        let _ = reader.read(&mut buf);
        dim = Dim::from_le_bytes(buf);

        let mut row = Vec::new();
        for _j in 0..dim {
            let _ = reader.read(&mut buf);
            let val: f32 = f32::from_le_bytes(buf);
            row.push(val);
        }
        dataset.data.insert(id, Rc::new(row));
    }
    dataset.dim = dim;

    println!("[Info] -> completed: {:?}", timer.elapsed());
    
    dataset

}


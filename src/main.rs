#[allow(unused_imports)]
use std::{
    env, 
    rc::Rc, 
    time, 
};

mod dataset_manager;
mod common;
mod search;
mod evaluation;

use crate::common::data::*;
use crate::dataset_manager::dataset_loader::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dataset_filename = args[1].clone();
    let query_filename = args[2].clone();
    let data_num: u64 = args[3].parse().unwrap();
    let k_for_search: usize = args[4].parse().unwrap();

    let try_num = 1;
    let loader = Fvecs;

    // use load instead of partial_load to load all dataset
    let dataset = loader.partial_load(&dataset_filename, data_num).unwrap();
    println!("[Info] dataset info: dim={}, num={}", dataset.dim, dataset.len());
    let queryset = loader.partial_load(&query_filename, 1).unwrap();
    println!("[Info] queries info: dim={}, num={}", queryset.dim, queryset.len());
    
    let using_index = search::IndexType::IVFFlat; // BruteForce, IVFFlat, HNSW
    let query = queryset.data.get(&0).unwrap();
    let answers = search::knn_search(using_index, Rc::clone(&query), k_for_search, &dataset);
    evaluation::evaluate_recall(answers.clone(), Rc::clone(&query), k_for_search, &dataset);

}

use crate::data;
use crate::index;
use std::time;

pub enum Index {
    BruteForce,
    IVFFlat,  
    HNSW, 
}

pub fn knn_search(using_index: Index, query: Vec<f32>, k: u32, data: &data::Dataset) -> Vec<data::Answer>{
    println!("[Info] kNN search: k={}", k);
    
    let timer = time::Instant::now();

    let mut answers = Vec::new();
    match using_index {
        Index::BruteForce => {
            answers = index::brute_force::knn(query, k, data);
        }, 
        Index::IVFFlat => {}, 
        Index::HNSW => {},
    }
    println!("[Info] -> completed: {:?}", timer.elapsed());
    
    answers

}
pub mod distance;
pub mod index;

use crate::common::data;
use crate::common::data::*;
use std::time;

pub enum IndexType {
    BruteForce,
    IVFFlat,  
    HNSW, 
}

pub fn knn_exact_search(query: VecData, k_for_search: usize, dataset: &Dataset) -> Answers {
    index::brute_force::knn(query, k_for_search, dataset)
}

pub fn knn_search(using_index: IndexType, query: VecData, k_for_search: usize, dataset: &Dataset) -> Answers {
    let mut answers = Vec::new();
    match using_index {
        IndexType::BruteForce => {
            println!("[Info] Use no index (brute-force search)");
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            answers = index::brute_force::knn(query, k_for_search, dataset);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        IndexType::IVFFlat => {
            println!("[Info] Use IVF Flat index (cluster-based)");
            let k_for_kmeans = 10;
            let kmeans_max_loop = 10;

            println!("[Info] build index: k for kmeans={}, max loop={}", k_for_kmeans, kmeans_max_loop);
            let timer = time::Instant::now();
            let index = index::ivf_flat::build(dataset, k_for_kmeans, kmeans_max_loop);
            println!("[Info] -> completed: {:?}", timer.elapsed());
            
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            answers = index::ivf_flat::knn(query, k_for_search, index);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        IndexType::HNSW => {
            println!("[Info] Use HNSW index (graph-based)");
        },
    }
    answers

}
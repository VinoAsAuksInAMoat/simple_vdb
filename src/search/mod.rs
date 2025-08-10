use crate::data;
use crate::index;
use std::time;

pub enum Index {
    BruteForce,
    IVFFlat,  
    HNSW, 
}

pub fn knn_exact_search(query: Vec<f32>, k_for_search: usize, data: &data::Dataset) -> Vec<data::Answer>{
    index::brute_force::knn(query, k_for_search, data)
}

pub fn knn_search(using_index: Index, query: Vec<f32>, k_for_search: usize, data: &data::Dataset) -> Vec<data::Answer>{
    let mut answers = Vec::new();
    match using_index {
        Index::BruteForce => {
            println!("[Info] Use no index (brute-force search)");
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            answers = index::brute_force::knn(query, k_for_search, data);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        Index::IVFFlat => {
            println!("[Info] Use IVF Flat index (cluster-based)");
            let k_for_kmeans = 10;
            let kmeans_max_loop = 10;

            println!("[Info] build index: k for kmeans={}, max loop={}", k_for_kmeans, kmeans_max_loop);
            let timer = time::Instant::now();
            let index = index::ivf_flat::build(data, k_for_kmeans, kmeans_max_loop);
            println!("[Info] -> completed: {:?}", timer.elapsed());
            
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            answers = index::ivf_flat::knn(query, k_for_search, index);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        Index::HNSW => {
            println!("[Info] Use HNSW index (graph-based)");
        },
    }
    answers

}
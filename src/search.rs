use std::rc::Rc;
use std::time;

pub mod distance;
pub mod index;

use crate::common::{
    data::datatypes::*, 
    data::neighbor::*, 
    data::search_result::*, 
};
use crate::search::index::interface::*;

#[allow(dead_code)]
pub enum IndexType {
    BruteForce,
    IVFFlat,  
    HNSW, 
}

pub fn knn_exact_search(query: Rc<VecData>, k_for_search: usize, dataset: &Dataset) -> SearchResult {
    let mut index = index::brute_force::Index();
    index.knn(dataset, query, k_for_search)
}

pub fn knn_search(using_index: IndexType, query: Rc<VecData>, k_for_search: usize, dataset: &Dataset) -> SearchResult {
    let mut ground_truth = Vec::new();
    match using_index {
        IndexType::BruteForce => {
            println!("[Info] Use no index (brute-force search)");
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            let mut index = index::brute_force::Index::build();
            ground_truth = index.knn(dataset, query, k_for_search);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        IndexType::IVFFlat => {
            println!("[Info] Use IVF Flat index (cluster-based)");
            let cluster_num: usize = 10;
            let kmeans_max_loop = 10;

            println!("[Info] build index: k for kmeans={}, max loop={}", cluster_num, kmeans_max_loop);
            let timer = time::Instant::now();
            let mut index = index::ivf_flat::Index::build(dataset, cluster_num, kmeans_max_loop);
            println!("[Info] -> completed: {:?}", timer.elapsed());
            
            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            ground_truth = index.knn(dataset, query, k_for_search);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        }, 
        IndexType::HNSW => {
            println!("[Info] Use HNSW index (graph-based)");
            let pg_max_degree: index::hnsw::Degree = 24;
            let num_layers: u8 = 1;
            let search_queue_size: usize = k_for_search + 10;

            let timer = time::Instant::now();
            let mut index = index::hnsw::Index::build(dataset, pg_max_degree, num_layers, search_queue_size);
            println!("[Info] -> completed: {:?}", timer.elapsed());

            println!("[Info] kNN search: k={}", k_for_search);
            let timer = time::Instant::now();
            ground_truth = index.knn(dataset, query, k_for_search);
            println!("[Info] -> completed: {:?}", timer.elapsed());
        },
    }
    println!("{:?}", ground_truth);
    ground_truth

}
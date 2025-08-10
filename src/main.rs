use std::env;

mod dataset_manager;
mod data;
mod distance;
mod index;
mod search;
mod evaluation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dataset_filename = args[1].clone();
    let query_filename = args[2].clone();
    let data_num: u64 = args[3].parse().unwrap();
    let k_for_search: usize = args[4].parse().unwrap();

    let dataset = dataset_manager::dataset_loader::load_fvecs(dataset_filename, data_num);
    println!("[Info] dataset info: dim={}, num={}", dataset.dim, dataset.num);
    let queries = dataset_manager::dataset_loader::load_fvecs(query_filename, 1);
    println!("[Info] queries info: dim={}, num={}", queries.dim, queries.num);

    //let using_index = search::Index::BruteForce;
    let using_index = search::Index::IVFFlat;
    let query = queries.data[0].vec.clone();
    let answers = search::knn_search(using_index, query.clone(), k_for_search, &dataset);
    evaluation::evaluate_search_performance(answers.clone(), query.clone(), k_for_search, &dataset);

}

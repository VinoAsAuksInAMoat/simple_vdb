use std::env;

mod dataset_manager;
mod data;
mod distance;
mod index;
mod search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let data_num: u64 = args[2].parse().unwrap();
    let search_k: u32 = args[3].parse().unwrap();

    let data = dataset_manager::dataset_loader::load_fvecs(filename, data_num);
    println!("[test] the dimension of data is: {}", data.dim);
    println!("[test] the number of data is: {}", data.num);
    println!("[test] one of the data is: {:?}", data.data[0].vec);

    let using_index = search::Index::BruteForce;
    let answers = search::knn_search(using_index, data.data[0].vec.clone(), search_k, &data);
    println!("{:?}", answers);

}

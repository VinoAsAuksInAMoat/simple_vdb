use crate::data;
use crate::search;

pub fn evaluate_search_performance(answers: Vec<data::Answer>, query: Vec<f32>, k_for_search: usize, dataset: &data::Dataset) {
    let exact_answers: Vec<data::Answer> = search::knn_exact_search(query.clone(), k_for_search, &dataset);
    let mut tp_count: u32 = 0;
    let mut answers_ids = Vec::new();
    let mut exact_answers_ids = Vec::new();
    for v in answers.iter() {
        answers_ids.push(v.id.clone());
    }
    for v in exact_answers.iter() {
        exact_answers_ids.push(v.id.clone());
    }
    for ele in exact_answers.iter() {
        if answers_ids.contains(&ele.id) {
            tp_count +=1;
        }
    }
    let recall: f64 = tp_count as f64 / exact_answers.len() as f64;
    println!("[Info] Search accuracy: recallK@K={}", recall);
    /*
    println!("Answers: \n{:?}", answers);
    println!("[test] answers_ids: {:?}", answers_ids);
    println!("[test] exact_answers_ids: {:?}", exact_answers_ids);
    */
}

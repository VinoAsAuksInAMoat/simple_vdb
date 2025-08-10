use crate::data;
use crate::search;

pub fn evaluate_recall(answers: Vec<data::Answer>, query: Vec<f32>, k_for_search: usize, dataset: &data::Dataset) {
    println!("[Info] Calculating search accuracy...");
    let exact_answers: Vec<data::Answer> = search::knn_exact_search(query.clone(), k_for_search, &dataset);

    let mut answers_ids = Vec::new();
    for v in answers.iter() {
        answers_ids.push(v.id.clone());
    }

    let mut tp_count: u32 = 0;
    for ele in exact_answers.iter() {
        if answers_ids.contains(&ele.id) {
            tp_count +=1;
        }
    }

    let recall: f64 = tp_count as f64 / exact_answers.len() as f64;
    println!("[Info] -> completed: recallK@K={} (K={})", recall, k_for_search);
}

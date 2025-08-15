use std::rc::Rc;

use crate::common::data;
use crate::common::data::*;
use crate::search;

pub fn evaluate_recall(answers: Answers, query: Rc<VecData>, k_for_search: usize, dataset: &data::Dataset) {
    println!("[Info] Calculating search accuracy...");
    let exact_answers: Answers = search::knn_exact_search(Rc::clone(&query), k_for_search, &dataset);

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

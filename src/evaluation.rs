use std::rc::Rc;

use crate::common::{
    data::*, 
    data::datatypes::*, 
    data::neighbor::*, 
    data::search_result::*, 
};
use crate::search;

pub fn evaluate_recall(search_result: SearchResult, query: Rc<VecData>, k_for_search: usize, dataset: &Dataset) {
    println!("[Info] Calculating search accuracy...");
    let ground_truth: SearchResult = search::knn_exact_search(Rc::clone(&query), k_for_search, &dataset);

    let mut answers_ids = Vec::new();
    for v in search_result.iter() {
        answers_ids.push(v.dataid.clone());
    }

    let mut tp_count: u32 = 0;
    for ele in ground_truth.iter() {
        if answers_ids.contains(&ele.dataid) {
            tp_count +=1;
        }
    }

    let recall: f64 = tp_count as f64 / ground_truth.len() as f64;
    println!("[Info] -> completed: recallK@K={} (K={})", recall, k_for_search);
}

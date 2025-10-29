use std::{collections::HashSet, rc::Rc};

use crate::common::data::*;
use crate::search;

pub fn evaluate_recall(
    search_result: &SearchResult,
    query: &VecData,
    k_for_search: usize,
    dataset: &Dataset,
) {
    println!("[Info] Calculating search accuracy...");
    let ground_truth: SearchResult = search::knn_exact_search(&query, k_for_search, &dataset);

    let mut answers_ids: HashSet<DataId> = search_result.iter().map(|v| v.dataid.clone()).collect();

    let tp_count = ground_truth
        .iter()
        .filter(|ele| answers_ids.contains(&ele.dataid))
        .count();

    let recall: f64 = tp_count as f64 / ground_truth.len() as f64;
    println!(
        "[Info] -> completed: recallK@K={} (K={})",
        recall, k_for_search
    );
}

use crate::common::{
    data::datatypes::*, 
    data::neighbor::*, 
};

pub type SearchResult = Vec<Neighbor>;

pub fn extract_topk(search_result: SearchResult, k: usize) -> SearchResult {
    let mut topk_search_result = search_result.clone();
    topk_search_result.sort();
    let _ = topk_search_result.split_off(k);
    topk_search_result
}

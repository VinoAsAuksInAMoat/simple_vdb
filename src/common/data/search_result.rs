use crate::common::{data::datatypes::*, data::neighbor::*};

pub type SearchResult = Vec<Neighbor>;

pub fn extract_topk(mut search_result: SearchResult, k: usize) -> SearchResult {
    if k < search_result.len() {
        let _ = search_result.select_nth_unstable(k);
        let mut topk = &mut search_result[..k];
        topk.sort_unstable();
        topk.to_vec()
    } else {
        search_result.sort_unstable();
        search_result
    }
}

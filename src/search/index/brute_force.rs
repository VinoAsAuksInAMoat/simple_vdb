use std::{collections::BinaryHeap, rc::Rc};

use crate::common::data::*;
use crate::search::{
    distance::interface::*, distance::l2distance::L2Distance, index::interface::*,
};

pub struct Index();

impl Index {
    pub fn build() -> Self {
        Index()
    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: &VecData, k: usize) -> SearchResult {
        let mut heap = BinaryHeap::with_capacity(k);
        for (dataid, vecdata) in &dataset.data {
            let dist = L2Distance::calc(&query, vecdata);
            if heap.len() < k {
                heap.push(Neighbor {
                    dataid: *dataid,
                    dist,
                });
            } else if dist < heap.peek().unwrap().dist {
                heap.pop();
                heap.push(Neighbor {
                    dataid: *dataid,
                    dist,
                });
            }
        }
        heap.into_sorted_vec()
    }
    fn knn_rc(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> SearchResult {
        let mut search_result: SearchResult = Vec::new();
        for (dataid, vecdata) in &dataset.data {
            let dist = L2Distance::calc(&query, vecdata);
            search_result.push(Neighbor {
                dataid: *dataid,
                dist: dist,
            });
        }
        extract_topk(search_result, k)
    }
}

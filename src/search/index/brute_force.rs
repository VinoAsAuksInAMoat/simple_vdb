use std::rc::Rc;

use crate::common::{
    data::datatypes::*, 
    data::neighbor::*, 
    data::search_result::*, 
};
use crate::search::{
    distance::interface::*,
    distance::l2distance::L2Distance, 
    index::interface::*
};

pub struct Index();

impl Index {
    pub fn build() -> Self { Index() }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> SearchResult {
        let mut search_result: SearchResult = Vec::new();
        for (dataid, vecdata) in &dataset.data {
            let dist = L2Distance::calc(Rc::clone(&query), Rc::clone(vecdata));
            search_result.push(Neighbor{
                dataid: *dataid, 
                dist: dist, 
            });
        }
        extract_topk(search_result, k)

    }

}
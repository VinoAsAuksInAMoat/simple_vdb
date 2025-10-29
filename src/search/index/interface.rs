use std::rc::Rc;

use crate::common::data::*;

pub trait AnnSearch {
    fn knn(&mut self, dataset: &Dataset, query: &VecData, k: usize) -> SearchResult;
    fn knn_rc(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> SearchResult;
}

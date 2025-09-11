use std::rc::Rc;

use crate::common::{
    data::datatypes::*, 
    data::neighbor::*, 
    data::search_result::*, 
};

pub trait AnnSearch {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> SearchResult;
}
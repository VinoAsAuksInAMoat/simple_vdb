use std::rc::Rc;

use crate::common::data::*;
use crate::search::{distance, index::interface::*};

pub struct Index();

impl Index {
    pub fn build() -> Self { Index() }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Answers {
        let mut answers: Answers = Vec::new();
        for (dataid, vecdata) in &dataset.data {
            let dist = distance::l2_distance(Rc::clone(&query), Rc::clone(vecdata));
            answers.push(Answer{
                id: *dataid, 
                dist: dist, 
            });
        }
        extract_topk(answers, k)

    }

}
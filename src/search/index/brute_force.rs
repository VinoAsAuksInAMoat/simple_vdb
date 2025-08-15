use std::rc::Rc;

use crate::common::data::*;
use crate::search::distance;

pub fn knn(query: Rc<VecData>, k: usize, dataset: &Dataset) -> Answers {
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
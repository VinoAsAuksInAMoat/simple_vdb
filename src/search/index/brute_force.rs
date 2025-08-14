use crate::common::data;
use crate::common::data::*;
use crate::search::distance;

pub fn knn(query: VecData, k: usize, dataset: &Dataset) -> Answers {
    let mut answers: Answers = Vec::new();
    let mut id = 0;
    for v in &dataset.data {
        let dist = distance::l2_distance(query.clone(), v.vec.clone());
        answers.push(Answer{
            id: id, 
            dist: dist, 
        });
        id += 1;
    }
    extract_topk(answers, k)

}
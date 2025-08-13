use crate::common::data;
use crate::search::distance;

pub fn knn(query: Vec<f32>, k: usize, dataset: &data::Dataset) -> Vec<data::Answer> {
    let mut answers: Vec<data::Answer> = Vec::new();
    let mut id = 0;
    for v in &dataset.data {
        let dist = distance::l2_distance(query.clone(), v.vec.clone());
        answers.push(data::Answer{
            id: id, 
            dist: dist, 
        });
        id += 1;
    }
    data::extract_topk(answers, k)

}
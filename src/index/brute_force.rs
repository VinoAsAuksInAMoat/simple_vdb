use crate::data;
use crate::distance;

pub fn knn(query: Vec<f32>, k: u32, dataset: &data::Dataset) -> Vec<data::Answer> {
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

    answers.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
    let _ = answers.split_off(k as usize);

    answers
}
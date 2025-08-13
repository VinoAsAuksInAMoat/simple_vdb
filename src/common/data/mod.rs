#[derive(Clone)]
pub struct Data {
    pub vec: Vec<f32>, 
    pub id: u64, 
}

pub struct Dataset {
    pub dim: u32, 
    pub num: u64, 
    pub data: Vec<Data>, 
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: u64, 
    pub dist: f32, 
}

pub fn extract_topk(answers: Vec<Answer>, k: usize) -> Vec<Answer> {
    let mut topk_answers = answers.clone();
    topk_answers.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
    let _ = topk_answers.split_off(k);
    topk_answers
}

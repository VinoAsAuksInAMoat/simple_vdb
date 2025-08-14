use std::rc::Rc;

pub type VecData = Vec<f32>;
pub type DataID = u64;
pub type Dim = u32;
pub type Dist = f32;

#[derive(Clone)]
pub struct Data {
    pub vec: VecData, 
    pub id: DataID, 
}

pub struct Dataset {
    pub dim: Dim, 
    pub num: u64, 
    pub data: Vec<Rc<Data>>, 
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: DataID, 
    pub dist: Dist, 
}

pub type Answers = Vec<Answer>;

pub fn extract_topk(answers: Answers, k: usize) -> Answers {
    let mut topk_answers = answers.clone();
    topk_answers.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
    let _ = topk_answers.split_off(k);
    topk_answers
}

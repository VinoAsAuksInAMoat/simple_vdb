use std::cmp::{Ordering};
use std::rc::Rc;
use std::collections::HashMap;

pub type VecData = Vec<f32>;
pub type DataID = u64;
pub type Dim = u32;
pub type Dist = f32;

pub struct Dataset {
    pub dim: Dim, 
    pub num: u64, 
    pub data: HashMap<DataID, Rc<VecData>>, 
}

#[derive(Debug, Clone)]
pub struct Neighbor {
    pub dataid: DataID, 
    pub dist: Dist, 
}

impl PartialEq for Neighbor {
    fn eq(&self, other: &Self) -> bool {
        self.dataid == other.dataid
    }
}

impl Eq for Neighbor {}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist.partial_cmp(&other.dist).unwrap().then(self.dataid.cmp(&other.dataid)))
    }
}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type Answers = Vec<Neighbor>;

pub fn extract_topk(answers: Answers, k: usize) -> Answers {
    let mut topk_answers = answers.clone();
    topk_answers.sort();
    let _ = topk_answers.split_off(k);
    topk_answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Neighbor_cmp() {
        let n1 = Neighbor {
            dataid: 10, 
            dist: 0.5, 
        };
        let n2 = Neighbor {
            dataid: 20, 
            dist: 0.3, 
        };
        let n3 = Neighbor {
            dataid: 10, 
            dist: 0.3, 
        };
        assert!(n2 < n1);
        assert!(n3 < n2);
        assert!(n3 < n1);
    }
}
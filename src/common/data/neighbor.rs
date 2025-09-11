use std::{
    rc::Rc, 
    collections::HashMap, 
    cmp::Ordering, 
};
use crate::common::data::datatypes::*;


#[derive(Debug, Clone)]
pub struct Neighbor {
    pub dataid: DataId, 
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
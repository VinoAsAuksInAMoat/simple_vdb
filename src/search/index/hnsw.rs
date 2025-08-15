use std::rc::Rc;
use std::collections::BTreeMap;

use crate::common::data::*;
use crate::search::{distance, index::interface::*};

pub type NodeID = u64; 
pub type Degree = u32;

pub struct Node{
    nodeid: NodeID, 
    dataid: DataID, 
    neighbors: Vec<(NodeID, Dist)>, 
}

pub struct Index {
    node_arena: BTreeMap<NodeID, Rc<Node>>, 
    largest_id: NodeID, 
}

impl Index {
    pub fn build() -> Self {
        todo!();
    }
    pub fn alloc(&mut self, dataid: DataID, neighbors: Vec<(NodeID, Dist)>) -> NodeID {
        let nodeid: NodeID = self.issue_id();
        let node = Node{
            nodeid: nodeid, 
            dataid: dataid, 
            neighbors: neighbors, 
        };
        self.node_arena.insert(nodeid, Rc::new(node));
        nodeid
    }
    fn issue_id(&mut self) -> NodeID {
        if self.node_arena.is_empty() == true {
            return 0;
        } else if (self.largest_id as u64) < ((usize::MAX-1) as u64) {
            self.largest_id += 1;
            return self.largest_id;
        } else {
            for id_cand in 0..(usize::MAX-1) as u64 {
                if self.node_arena.contains_key(&id_cand) == false {
                    return id_cand as NodeID;
                } 
            }
            panic!("[Error] id allocation is failed: lack of id number");
        }
    }
    fn find_knn(&self, cmp_data: Rc<VecData>, dataset: &Dataset, pg_max_degree: Degree) {
        todo!();
    }
    pub fn build_naive_pg(dataset: &Dataset, pg_max_degree: Degree) -> Self {
        let mut index = Index {
            node_arena: BTreeMap::new(), 
            largest_id: 0, 
        };
        for (dataid, vecdata) in dataset.data.iter() {
            let neighbors = index.find_knn_naive(dataset, Rc::clone(vecdata), pg_max_degree as usize);
            let nodeid = index.alloc(*dataid, neighbors);
        }
        index
    }
    fn find_knn_naive(&self, dataset: &Dataset, cmp_vecdata: Rc<VecData>, k: usize) -> Vec<(NodeID, Dist)> {
        let mut neighbors: Vec<(NodeID, Dist)> = Vec::new();
        for (dataid, vecdata) in &dataset.data {
            let dist = distance::l2_distance(Rc::clone(&cmp_vecdata), Rc::clone(vecdata));
            neighbors.push((*dataid, dist));
        }
        self.extract_topk_pg(neighbors, k)

    }
    fn extract_topk_pg(&self, neighbors: Vec<(NodeID, Dist)>, k: usize) -> Vec<(NodeID, Dist)> {
        let mut topk_answers = neighbors.clone();
        topk_answers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let _ = topk_answers.split_off(k);
        topk_answers
    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Answers {
        todo!();
    }
}



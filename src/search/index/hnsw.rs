use std::rc::Rc;
use std::collections::BTreeMap;

use crate::common::data::*;
use crate::search::{distance, index::interface::*};

pub type NodeID = u64; 
pub type Degree = u32;
pub type LayerID = u8;

pub struct Node{
    nodeid: NodeID, 
    dataid: DataID, 
    neighbors: Vec<Neighbor>, 
}

pub struct Layer {
    node_arena: BTreeMap<NodeID, Rc<Node>>, 
    largest_id: NodeID, 
}

pub struct Index {
    layers: Vec<Layer>, 
}

impl Layer {
    pub fn alloc(&mut self, dataid: DataID, neighbors: Vec<Neighbor>) -> NodeID {
        let nodeid: NodeID = self.issue_nodeid();
        let node = Node{
            nodeid: nodeid, 
            dataid: dataid, 
            neighbors: neighbors, 
        };
        self.node_arena.insert(nodeid, Rc::new(node));
        nodeid
    }
    fn issue_nodeid(&mut self) -> NodeID {
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
        let mut layer = Layer {
            node_arena: BTreeMap::new(), 
            largest_id: 0, 
        };
        for (dataid, vecdata) in dataset.data.iter() {
            let neighbors = layer.find_knn_naive(dataset, Rc::clone(vecdata), pg_max_degree as usize);
            let nodeid = layer.alloc(*dataid, neighbors);
        }
        layer
    }
    fn find_knn_naive(&self, dataset: &Dataset, cmp_vecdata: Rc<VecData>, k: usize) -> Vec<Neighbor> {
        let mut neighbors: Vec<Neighbor> = Vec::new();
        for (dataid, vecdata) in &dataset.data {
            let dist = distance::l2_distance(Rc::clone(&cmp_vecdata), Rc::clone(vecdata));
            neighbors.push(Neighbor{
                dataid: *dataid, 
                dist: dist, 
            });
        }
        self.extract_topk_pg(neighbors, k)

    }
    fn extract_topk_pg(&self, neighbors: Vec<Neighbor>, k: usize) -> Vec<Neighbor> {
        let mut topk_answers = neighbors.clone();
        topk_answers.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
        let _ = topk_answers.split_off(k);
        topk_answers
    }
}

impl Index {
    pub fn build(dataset: &Dataset, pg_max_degree: Degree) -> Self{
        Self { 
            layers: vec![Layer::build_naive_pg(dataset, pg_max_degree)], 
        }
    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Answers {
        todo!();
    }
}



use std::{
    rc::Rc, 
    collections::{BTreeMap, BinaryHeap, HashSet}, 
    cmp::{Reverse, min}, 
};
use rand::{
    seq::SliceRandom, 
    prelude::*, 
};

use crate::{
    common::data::*, 
    search::{
        distance, 
        index::interface::*
    }, 
};

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
    largest_id: NodeID,  // bitmapで管理に変更？
}

pub struct Index {
    layers: BTreeMap<LayerID, Layer>, 
    entry_point: NodeID, 
    search_queue_size: usize, 
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
    fn search_layer(&self, dataset: &Dataset, query: Rc<VecData>, start_point: Neighbor, search_queue_size: usize) -> BinaryHeap<Neighbor> {
        let mut visited: Vec<NodeID> = vec![start_point.dataid];
        let mut candidates: BinaryHeap<Reverse<Neighbor>> = BinaryHeap::from([Reverse(start_point.clone())]);
        let mut neighbors: BinaryHeap<Neighbor> = BinaryHeap::from([start_point.clone()]);
        while candidates.is_empty() == false {
            let current_point = candidates.pop().unwrap().0;
            let farghest_neighbor = neighbors.peek().unwrap();
            if current_point.dist > farghest_neighbor.dist {
                break;
            }
            for neighbor_ele in self.node_arena[&current_point.dataid].neighbors.iter() {
                if visited.iter().any(|x| *x == neighbor_ele.dataid) == true {
                    continue;
                }
                let cmp_neighbor = Neighbor{
                    dataid: neighbor_ele.dataid, 
                    dist: distance::l2_distance(Rc::clone(&query), Rc::clone(&dataset.data[&neighbor_ele.dataid])), 
                };
                visited.push(cmp_neighbor.dataid);
                let farghest_neighbor = neighbors.peek().unwrap();
                if cmp_neighbor.dist < farghest_neighbor.dist || neighbors.len() < search_queue_size {
                    candidates.push(Reverse(cmp_neighbor.clone()));
                    neighbors.push(cmp_neighbor.clone());
                    if neighbors.len() > search_queue_size {
                        neighbors.pop();
                    }
                }
            }
        }
        neighbors
    }
    pub fn build_naive_pg(dataset: &Dataset, dataid_set: Vec<DataID>, pg_max_degree: Degree) -> Self {
        let mut layer = Layer {
            node_arena: BTreeMap::new(), 
            largest_id: 0, 
        };
        for dataid in dataid_set.iter() {
            let vecdata = &dataset.data[dataid];
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
        extract_topk(neighbors, k)

    }
}

impl Index {
    pub fn build(dataset: &Dataset, pg_max_degree: Degree, num_layers: u8, search_queue_size: usize) -> Self {
        // build a layer: layer id = 0
        let mut layers = BTreeMap::new();
        let mut dataid_set: Vec<DataID> = dataset.data.clone().into_keys().collect();
        layers.insert(0, Layer::build_naive_pg(dataset, dataid_set.clone(), pg_max_degree));
        if num_layers == 1 {
            return Self { 
                layers: layers, 
                entry_point: dataid_set[0], // todo: add error handling (e.g. len==0)
                search_queue_size: search_queue_size, 
            };
        }
        // build layers: layer id = 1, ... num_layers-1
        for layerid in 1..num_layers-1 {
            let sampling_num: usize = dataid_set.len() as usize / 10 as usize;
            if sampling_num == 0 {
                break;
            }
            dataid_set = Self::sampling(dataid_set.clone(), sampling_num);
            layers.insert(layerid as u8, Layer::build_naive_pg(dataset, dataid_set.clone(), pg_max_degree));
        }
        // build a layer: layer id = num_layers
        dataid_set = Self::sampling(dataid_set.clone(), 1);
        layers.insert((num_layers-1) as u8, Layer::build_naive_pg(dataset, dataid_set.clone(), pg_max_degree));
        Self { 
            layers: layers, 
            entry_point: dataid_set[0], // todo: add error handling (e.g. len==0)
            search_queue_size: search_queue_size, 
        }
    }
    fn sampling(dataid_set: Vec<DataID>, sample_num: usize) -> Vec<DataID> {
        if dataid_set.len() < sample_num {
            return dataid_set;
        }
        let mut sampled = HashSet::new();
        let mut rng: ThreadRng = rand::thread_rng();
        while sampled.len() < sample_num {
            let ele: DataID = rng.gen_range(0..dataid_set.len() as u64);
            sampled.insert(ele);
        }
        Vec::from_iter(sampled)
    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Answers {
        let mut start_point = Neighbor{
            dataid: self.entry_point, 
            dist: distance::l2_distance(Rc::clone(&query), Rc::clone(&dataset.data[&self.entry_point]))
        };
        for layerid in self.layers.len()-1..1{
            let neighbors = self.layers[&(layerid as u8)].search_layer(dataset, Rc::clone(&query), start_point, 1);
            let nearest = extract_topk(neighbors.into_sorted_vec().clone(), 1);
            start_point = nearest[0].clone();
        }

        let neighbors = self.layers[&0].search_layer(dataset, Rc::clone(&query), start_point, self.search_queue_size);
        let answers = neighbors.into_sorted_vec();
        extract_topk(answers.clone(), min(answers.len(), k))
    }
}



use std::rc::Rc;

use crate::common::data::*;
use crate::search::{
    distance::interface::*,
    distance::l2distance::L2Distance, 
    index::interface::*
};

const INF_F32: f32 = f32::INFINITY;

#[derive(Clone)]
struct Cluster {
    centroid: Rc<VecData>, // Rcなしより7%高速
    vectors: Vec<DataId>, 
    data_num: u64, 
}

impl Cluster {
    fn calc_centroid(&mut self, dataset: &Dataset) {
        if dataset.len() == 0 {
            return;
        }
        let mut sum: VecData = vec![0.0; dataset.dim as usize];
        for data_id in self.vectors.iter() {
            for (v_in_vecdata, v_in_sum) in dataset.data.get(data_id).unwrap().iter().zip(sum.iter_mut()) {
                *v_in_sum += *v_in_vecdata;
            }
        }
        let num_f32 = dataset.len() as f32;
        let new_centroid = sum.iter().map(|x| x / num_f32).collect();
        self.centroid = Rc::new(new_centroid);
    } 
}

#[derive(Clone)]
pub struct Index {
    clusters: Vec<Cluster>, 
}

impl Index {
    pub fn build(dataset: &Dataset, cluster_num: usize, kmeans_max_loop: u32) -> Self {
        // init
        let mut index = Index {
            clusters: vec![Cluster{
                centroid: Rc::new(Vec::new()), 
                vectors: Vec::new(), 
                data_num: 0, 
            }; cluster_num], 
        };
        let mut prev_index = Index {
            clusters: vec![Cluster{
                centroid: Rc::new(Vec::new()), 
                vectors: Vec::new(), 
                data_num: 0, 
            }; cluster_num], 
        };
        for (cluster, (_dataid, vecdata)) in prev_index.clusters.iter_mut().zip(dataset.data.iter()) {
            cluster.centroid = Rc::clone(vecdata);
        }

        for loop_count in 0..kmeans_max_loop {
            index = Index {
                clusters: vec![Cluster{
                    centroid: Rc::new(Vec::new()), 
                    vectors: Vec::new(), 
                    data_num: 0, 
                }; cluster_num], 
            };
            
            for (data_id, vecdata) in dataset.data.iter() {
                let min_cluster_id = prev_index.find_nearest_cluster(Rc::clone(vecdata));

                index.clusters[min_cluster_id].vectors.push(*data_id);
                index.clusters[min_cluster_id].data_num += 1;
            }
            for cluster in index.clusters.iter_mut() {
                cluster.calc_centroid(dataset);
            }
            for (clusters_cur, clusters_prev) in index.clusters.iter_mut().zip(prev_index.clusters.iter()){
                if clusters_cur.data_num == 0 {
                    clusters_cur.centroid = Rc::clone(&clusters_prev.centroid);
                }
            }
            /*
            for (cid, cluster) in index.clusters.iter().enumerate() {
                println!("cluster{} has {} data", cid, cluster.data_num);
            }
            */
            
            let mut converged: bool = true;
            for (clusters_cur, clusters_prev) in index.clusters.iter().zip(prev_index.clusters.iter()){
                if *clusters_cur.vectors != *clusters_prev.vectors {
                    converged = false;
                    break;
                }
            }
            if converged == true {
                println!("[Test] the number of loops for kmeans = {}", loop_count);
                break;
            }
            prev_index = index.clone();
        }
        index

    }

    fn find_nearest_cluster(&mut self, cmp_vecdata: Rc<VecData>) -> usize {
        let mut min_dist = INF_F32;
        let mut min_cluster_id: usize = 0; 
        for cluster_id in 0..self.clusters.len() as usize {
            let dist = L2Distance::calc(&cmp_vecdata, &self.clusters[cluster_id].centroid);
            if dist < min_dist {
                min_dist = dist;
                min_cluster_id = cluster_id;
            }
        }
        min_cluster_id

    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Vec<Neighbor> {

        let min_cluster_id = self.find_nearest_cluster(Rc::clone(&query));

        let mut search_result: SearchResult = Vec::new();
        for data_id in self.clusters[min_cluster_id].vectors.iter() {
            let dist = L2Distance::calc(&query, dataset.data.get(data_id).unwrap());
            search_result.push(Neighbor{
                dist: dist, 
                dataid: *data_id, 
            });
        }

        let dist_calc_num = self.clusters[min_cluster_id].vectors.len() as u32 + self.clusters.len() as u32;
        println!("[Details] the num of dist calc: {}", dist_calc_num);
        extract_topk(search_result, k)
    }

}
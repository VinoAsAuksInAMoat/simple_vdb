use std::rc::Rc;

use crate::common::data::*;
use crate::search::{
    distance::interface::*, distance::l2distance::L2Distance, index::interface::*,
};

const INF_DIST: Dist = Dist::INFINITY;

#[derive(Clone)]
struct Cluster {
    centroid: VecData,
    ids: Vec<DataId>,
}

impl Cluster {
    fn calc_centroid(&mut self, dataset: &Dataset) {
        if dataset.len() == 0 {
            return;
        }
        let mut sum: VecData = vec![0.0; dataset.dim() as usize];
        for data_id in self.ids.iter() {
            let v = dataset.data.get(data_id).unwrap();
            for (v_in_vecdata, v_in_sum) in v.iter().zip(sum.iter_mut()) {
                *v_in_sum += *v_in_vecdata;
            }
        }
        let num_f32 = dataset.len() as f32;
        let new_centroid: VecData = sum.iter().map(|x| x / num_f32).collect();
        self.centroid = new_centroid.clone();
    }
    fn len(&self) -> usize {
        self.ids.len()
    }
    fn push(&mut self, dataid: DataId) {
        self.ids.push(dataid);
    }
}

#[derive(Clone)]
pub struct Index {
    clusters: Vec<Cluster>,
}

impl Index {
    pub fn build(dataset: &Dataset, cluster_num: usize, kmeans_max_loop: u32) -> Self {
        if dataset.data.is_empty() {
            panic!();
        }
        // init
        let dim = dataset.data[&0].len();
        let mut index = Index {
            clusters: Vec::new(),
        };
        let mut prev_index = Index {
            clusters: Vec::new(),
        };
        prev_index.init_centroids(dataset, cluster_num);
        prev_index
            .clusters
            .iter_mut()
            .for_each(|cluster| cluster.calc_centroid(dataset));

        for loop_count in 0..kmeans_max_loop {
            index = Index {
                clusters: vec![
                    Cluster {
                        centroid: Vec::new(),
                        ids: Vec::new(),
                    };
                    cluster_num
                ],
            };

            // 最近傍クラスタに割り振り
            dataset.data.iter().for_each(|(dataid, vecdata)| {
                let min_cluster_id = prev_index.find_nearest_cluster(vecdata);
                index.clusters[min_cluster_id].push(*dataid);
            });

            // centroidを計算
            index
                .clusters
                .iter_mut()
                .for_each(|cluster| cluster.calc_centroid(dataset));

            index
                .clusters
                .iter_mut()
                .zip(prev_index.clusters.iter())
                .for_each(|(clusters_cur, clusters_prev)| {
                    if clusters_cur.len() == 0 {
                        clusters_cur.centroid = clusters_prev.centroid.clone();
                    }
                });

            for (cid, cluster) in index.clusters.iter().enumerate() {
                println!("cluster{} has {} data", cid, cluster.len());
            }

            let mut converged: bool = true;
            for (clusters_cur, clusters_prev) in
                index.clusters.iter().zip(prev_index.clusters.iter())
            {
                if *clusters_cur.ids != *clusters_prev.ids {
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

    fn init_centroids(&mut self, dataset: &Dataset, cluster_num: usize) {
        let dim = dataset.data[&0].len();
        self.clusters = Vec::with_capacity(cluster_num);
        for (i, (dataid, data)) in dataset.data.iter().enumerate() {
            if i < cluster_num {
                self.clusters.push(Cluster {
                    centroid: data.to_vec(),
                    ids: vec![*dataid],
                })
            } else {
                let nearest_cluster_id: usize = self.find_nearest_cluster(data);
                self.clusters[nearest_cluster_id].push(*dataid);
            }
        }
    }

    fn find_nearest_cluster(&mut self, cmp_vecdata: &VecData) -> usize {
        let mut min_dist = INF_DIST;
        let mut min_cluster_id: usize = 0;
        self.clusters
            .iter()
            .enumerate()
            .for_each(|(cluster_id, cluster)| {
                let dist = L2Distance::calc(&cmp_vecdata, &(cluster.centroid));
                if dist < min_dist {
                    min_dist = dist;
                    min_cluster_id = cluster_id;
                }
            });
        min_cluster_id
    }
}

impl AnnSearch for Index {
    fn knn(&mut self, dataset: &Dataset, query: &VecData, k: usize) -> Vec<Neighbor> {
        todo!();
    }
    fn knn_rc(&mut self, dataset: &Dataset, query: Rc<VecData>, k: usize) -> Vec<Neighbor> {
        let min_cluster_id = self.find_nearest_cluster(&query);

        let mut search_result: SearchResult = Vec::new();
        for data_id in self.clusters[min_cluster_id].ids.iter() {
            let dist = L2Distance::calc(&query, dataset.data.get(data_id).unwrap());
            search_result.push(Neighbor {
                dist: dist,
                dataid: *data_id,
            });
        }

        let dist_calc_num =
            self.clusters[min_cluster_id].ids.len() as u32 + self.clusters.len() as u32;
        println!("[Details] the num of dist calc: {}", dist_calc_num);
        extract_topk(search_result, k)
    }
}

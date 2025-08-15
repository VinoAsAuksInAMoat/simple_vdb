use std::rc::Rc;

use crate::common::data::*;
use crate::search::distance;

const INF_F32: f32 = 100_000_000_000.0;

#[derive(Clone)]
struct Cluster {
    centroid: Rc<VecData>, // Rcなしより7%高速
    vectors: Vec<DataID>, 
    data_num: u64, 
}

#[derive(Clone)]
pub struct Index {
    clusters: Vec<Cluster>, 
}


pub fn build(dataset: &Dataset, cluster_num: usize, kmeans_max_loop: u32) -> Index {
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
            let min_cluster_id = find_nearest_cluster(Rc::clone(vecdata), &prev_index.clusters);

            index.clusters[min_cluster_id].vectors.push(*data_id);
            index.clusters[min_cluster_id].data_num += 1;
        }
        for cluster in index.clusters.iter_mut() {
            cluster.centroid = calc_centroid(dataset, &cluster.vectors, dataset.dim, dataset.num);
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

pub fn knn(dataset: &Dataset, query: Rc<VecData>, k: usize, index: Index) -> Vec<Answer> {

    let min_cluster_id = find_nearest_cluster(Rc::clone(&query), &index.clusters);

    let mut answers: Answers = Vec::new();
    for data_id in index.clusters[min_cluster_id].vectors.iter() {
        let dist = distance::l2_distance(Rc::clone(&query), Rc::clone(dataset.data.get(data_id).unwrap()));
        answers.push(Answer{
            dist: dist, 
            id: *data_id, 
        });
    }

    let dist_calc_num = index.clusters[min_cluster_id].vectors.len() as u32 + index.clusters.len() as u32;
    println!("[Details] the num of dist calc: {}", dist_calc_num);
    extract_topk(answers, k)
}

fn find_nearest_cluster(cmp_vecdata: Rc<VecData>, clusters: &Vec<Cluster>) -> usize {
    let mut min_dist = INF_F32;
    let mut min_cluster_id: usize = 0; 
    for cluster_id in 0..clusters.len() as usize {
        let dist = distance::l2_distance(Rc::clone(&cmp_vecdata), Rc::clone(&clusters[cluster_id].centroid));
        if dist < min_dist {
            min_dist = dist;
            min_cluster_id = cluster_id;
        }
    }
    min_cluster_id

}

fn calc_centroid(dataset: &Dataset, data_ids: &Vec<DataID>, dim: Dim, num: u64) -> Rc<VecData> {
    if num == 0 {
        return Rc::new(vec![0.0; dim as usize]);
    }
    let mut sum: VecData = vec![0.0; dim as usize];
    for data_id in data_ids.iter() {
        for (v_in_vecdata, v_in_sum) in dataset.data.get(data_id).unwrap().iter().zip(sum.iter_mut()) {
            *v_in_sum += *v_in_vecdata;
        }
    }
    let centroid = sum.iter().map(|x| x / num as f32).collect();
    Rc::new(centroid)
} 

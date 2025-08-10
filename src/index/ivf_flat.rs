use crate::data;
use crate::distance;

const INF_F32: f32 = 100_000_000_000.0;

#[derive(Clone)]
struct Cluster {
    centroid: Vec<f32>, 
    vectors: Vec<data::Data>, // ids 
    data_num: u64, 
}

#[derive(Clone)]
pub struct Index {
    clusters: Vec<Cluster>, 
}


pub fn build(dataset: &data::Dataset, k_for_kmeans: u32, kmeans_max_loop: u32) -> Index {
    // init
    let mut index = Index {
        clusters: vec![Cluster{
            centroid: Vec::new(), 
            vectors: Vec::new(), 
            data_num: 0, 
        }; 128], 
    };
    let mut prev_index = Index {
        clusters: vec![Cluster{
            centroid: Vec::new(), 
            vectors: Vec::new(), 
            data_num: 0, 
        }; 128], 
    };
    for cluster_id in 0..k_for_kmeans as usize{
        prev_index.clusters[cluster_id].centroid = dataset.data[cluster_id].vec.clone();
    }

    for loop_count in 0..kmeans_max_loop {
        index = Index {
            clusters: vec![Cluster{
                centroid: Vec::new(), 
                vectors: Vec::new(), 
                data_num: 0, 
            }; 128], 
        };
        for v in dataset.data.iter() {
            let mut min_dist = INF_F32;
            let mut min_cluster: usize = 0;
            for cluster_id in 0..k_for_kmeans as usize {
                let dist = distance::l2_distance(v.vec.clone(), prev_index.clusters[cluster_id].centroid.clone());
                if dist < min_dist {
                    min_dist = dist;
                    min_cluster = cluster_id;
                }
            }
            index.clusters[min_cluster].vectors.push(v.clone());
            index.clusters[min_cluster].data_num += 1;
        }
        for i in 0..index.clusters.len() {
            index.clusters[i].centroid = calc_centroid(&index.clusters[i].vectors, dataset.dim, dataset.num);
        }
        if index.clusters[0].centroid == prev_index.clusters[0].centroid {
            println!("[Test] loop_count = {}", loop_count);
            break;
        }
        prev_index = index.clone();
    }

    index

}

pub fn knn(query: Vec<f32>, k: usize, index: Index) -> Vec<data::Answer> {
    let mut min_dist = INF_F32;
    let mut min_cluster: usize = 0; 
    for cluster_id in 0..index.clusters.len() as usize {
        let dist = distance::l2_distance(query.clone(), index.clusters[cluster_id].centroid.clone());
        if dist < min_dist {
            min_dist = dist;
            min_cluster = cluster_id;
        }
    }

    let mut answers: Vec<data::Answer> = Vec::new();
    for v in index.clusters[min_cluster].vectors.iter() {
        let dist = distance::l2_distance(query.clone(), v.vec.clone());
        answers.push(data::Answer{
            dist: dist, 
            id: v.id.clone(), 
        });
    }

    let dist_calc_num = index.clusters[min_cluster].vectors.len() as u32 + index.clusters.len() as u32;
    println!("[Details] the num of dist calc: {}", dist_calc_num);
    data::extract_topk(answers, k)
}



fn calc_centroid(data: &Vec<data::Data>, dim: u32, num: u64) -> Vec<f32> {
    let dim: usize = dim as usize;
    if num == 0 {
        return vec![0.0; dim];
    }
    let mut sum: Vec<f32> = vec![0.0; dim];
    for v in data {
        for d in 0..dim as usize {
            sum[d] += v.vec[d];
        }
    }
    let mut centroid: Vec<f32> = vec![0.0; dim];
    for d in 0..dim as usize {
        centroid[d] = sum[d] / num as f32;
    }
    centroid
} 

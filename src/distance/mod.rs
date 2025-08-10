pub fn l2_distance(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
    if v1.len() != v2.len() {
        panic!("[Error] the dimensions of given vectors are not same");
    }

    let dim = v1.len();
    let mut pow_sum: f32 = 0.0;
    for i in 0..dim {
        let diff = v1[i] - v2[i];
        pow_sum += diff * diff;
    }
    pow_sum.sqrt()

}
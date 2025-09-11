use std::rc::Rc;

use crate::common::data::datatypes::{VecData, Dist};


use crate::search::distance::interface::*;

pub struct L2Distance;

impl DistanceCalculation for L2Distance {
    fn calc(v1: Rc<VecData>, v2: Rc<VecData>) -> Dist {
        if v1.len() != v2.len() { // compare dimensions of vectors
            panic!("[Error] the dimensions of given vectors are not same");
        }

        let mut pow_sum: Dist = 0.0;
        for (vec1, vec2) in v1.iter().zip(v2.iter()) { // mapは1.5倍時間がかかる
            let diff: Dist = vec1 - vec2;
            pow_sum += diff * diff; // powfは1.6倍時間がかかるのでNG
        }
        pow_sum.sqrt()

    }
}
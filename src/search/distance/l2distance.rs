#![feature(portable_simd)]
use std::simd::{f32x8, Simd, num::SimdFloat};

use crate::common::data::datatypes::{VecVal, VecData, Dist};
use crate::search::distance::interface::*;


pub struct L2Distance;

impl DistanceCalculation for L2Distance {
    fn calc(v1: &[VecVal], v2: &[VecVal]) -> Dist {

        Self::calc_simd(v1, v2)

        /* 

        // compare dimensions of vectors
        assert_eq!(v1.len(), v2.len(), "[Error] the dimensions of given vectors are not same");

        // calc
        let mut pow_sum: Dist = 0.0;
        for (vec1, vec2) in v1.iter().zip(v2.iter()) { // mapは1.5倍時間がかかる
            let diff: Dist = vec1 - vec2;
            pow_sum += diff * diff; // powfは1.6倍時間がかかる
        }
        pow_sum.sqrt()
         */

    }
    fn calc_simd(v1: &[VecVal], v2: &[VecVal]) -> Dist {
        // compare dimensions of vectors
        assert_eq!(v1.len(), v2.len(), "[Error] the dimensions of given vectors are not same");

        // calc with simd
        let mut pow_sum: Dist = 0.0;
        for i in (0..(v1.len())).step_by(8) {
            let vec1: Simd<f32, 8> = Simd::<f32, 8>::from_slice(&v1[i..(i+8)]);
            let vec2: Simd<f32, 8> = Simd::<f32, 8>::from_slice(&v2[i..(i+8)]);
            let diff: Simd<f32, 8> = vec1 - vec2;
            pow_sum += (diff * diff).reduce_sum();
        }
        pow_sum.sqrt()
    }
}
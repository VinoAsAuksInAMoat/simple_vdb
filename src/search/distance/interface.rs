use std::rc::Rc;

use crate::common::data::datatypes::{VecVal, VecData, Dist};


pub trait DistanceCalculation {
    fn calc(v1: &[VecVal], v2: &[VecVal]) -> Dist;
    fn calc_simd(v1: &[VecVal], v2: &[VecVal]) -> Dist;
}
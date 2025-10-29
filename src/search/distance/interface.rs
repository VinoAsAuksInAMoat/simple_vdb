use std::rc::Rc;

use crate::common::data::datatypes::{Dist, VecData, VecVal};

pub trait DistanceCalculation {
    fn calc(v1: &[VecVal], v2: &[VecVal]) -> Dist;
    fn calc_simd(v1: &[VecVal], v2: &[VecVal]) -> Dist;
}

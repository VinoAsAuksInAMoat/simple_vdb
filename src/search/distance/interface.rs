use std::rc::Rc;

use crate::common::data::datatypes::{VecData, Dist};


pub trait DistanceCalculation {
    fn calc(v1: Rc<VecData>, v2: Rc<VecData>) -> Dist;
}
pub struct Data {
    pub vec: Vec<f32>, 
    pub id: u64, 
}

pub struct Dataset {
    pub dim: u32, 
    pub num: u64, 
    pub data: Vec<Data>, 
}

#[derive(Debug)]
pub struct Answer {
    pub id: u64, 
    pub dist: f32, 
}

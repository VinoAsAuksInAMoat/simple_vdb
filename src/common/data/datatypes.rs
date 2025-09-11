use std::rc::Rc;
use std::collections::HashMap;

pub type VecData = Vec<f32>;
pub type DataId = u64;
pub type Dim = u32;
pub type Dist = f32;

pub struct Dataset {
    pub dim: Dim, 
    pub num: u64, 
    pub data: HashMap<DataId, Rc<VecData>>, 
    last_used_dataid: DataId, 
}

impl Dataset {
    pub fn new(dim_: Dim) -> Dataset {
        Dataset {
            dim: dim_, 
            num: 0, 
            data: HashMap::new(), 
            last_used_dataid: 0, 
        }
    }
    pub fn add(&mut self, vecdata: &VecData) -> DataId{
        let dataid = self.issue_dataid();
        self.data.insert(dataid, Rc::new(vecdata.clone()));
        self.num += 1;
        self.last_used_dataid = dataid;
        dataid
    }
    fn issue_dataid(&mut self) -> DataId {
        // todo: impl mutex
        if self.data.is_empty() == true {
            return 0;
        } else if (self.last_used_dataid as u64) < ((usize::MAX-1) as u64) {
            return self.last_used_dataid + 1;
        } else {
            for id_cand in 0..(usize::MAX-1) as u64 {
                if self.data.contains_key(&id_cand) == false {
                    return id_cand as DataId;
                } 
            }
            panic!("[Error] data id issue is failed: lack of data id");
        }
    }
}
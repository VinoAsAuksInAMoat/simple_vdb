use std::{
    collections::HashMap,
    rc::Rc,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

use crate::common::data::datatypes::*;
use crate::common::data::id_allocator::*;

pub struct Dataset {
    pub data: HashMap<DataId, Rc<VecData>>,
    dataid_allocator: Arc<IdAllocator>,
}

impl Dataset {
    pub fn new(dim: Dim) -> Dataset {
        Dataset {
            data: HashMap::new(),
            dataid_allocator: Arc::new(IdAllocator::new()),
        }
    }
    pub fn with_capacity(dim: Dim, capacity: usize) -> Dataset {
        Dataset {
            data: HashMap::with_capacity(capacity),
            dataid_allocator: Arc::new(IdAllocator::new()),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn dim(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.iter().len()
        }
    }
    pub fn add(&mut self, vecdata: VecData) -> DataId {
        let dataid = self.dataid_allocator.allocate();
        self.data.insert(dataid, Rc::new(vecdata));
        dataid
    }
    pub fn remove(&mut self, dataid: &DataId) {
        if self.contains_dataid(dataid) == false {
            return;
        }
        self.data.remove(dataid);
    }
    fn contains_dataid(&self, dataid: &DataId) -> bool {
        self.data.contains_key(dataid)
    }
}

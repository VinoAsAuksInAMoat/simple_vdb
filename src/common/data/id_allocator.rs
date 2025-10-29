use crossbeam_queue::SegQueue;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

use crate::common::data::datatypes;
use crate::common::data::datatypes::*;

pub struct IdAllocator {
    next_id: AtomicU64,
    released_ids: SegQueue<DataId>,
}

impl IdAllocator {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(0),
            released_ids: SegQueue::new(),
        }
    }
    pub fn allocate(&self) -> u64 {
        if let Some(id) = self.released_ids.pop() {
            id
        } else {
            let id = self.next_id.fetch_add(1, Ordering::Relaxed);
            if id > usize::MAX as u64 {
                panic!("[Error] data id issue is failed: lack of data id");
            }
            id
        }
    }
    pub fn release(&self, id: DataId) {
        self.released_ids.push(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_sequential_numbers_when_released_ids_empty() {
        let id_allocator = IdAllocator::new();

        // 新規IDを発行
        let id0 = id_allocator.allocate();
        assert_eq!(id0, 0);

        let id1 = id_allocator.allocate();
        assert_eq!(id1, 1);
        let id2 = id_allocator.allocate();
        assert_eq!(id2, 2);
        let id3 = id_allocator.allocate();
        assert_eq!(id3, 3);
    }

    #[test]
    fn recycles_released_numbers_when_released_ids_not_empty() {
        let id_allocator = IdAllocator::new();

        // 新規IDを発行
        let id0 = id_allocator.allocate();
        assert_eq!(id0, 0);

        let id1 = id_allocator.allocate();
        assert_eq!(id1, 1);
        let id2 = id_allocator.allocate();
        assert_eq!(id2, 2);
        let id3 = id_allocator.allocate();
        assert_eq!(id3, 3);

        // id1を削除
        id_allocator.release(id1);
        // 値3を削除
        id_allocator.release(3);

        // 再度IDを発行 -> リリースされた3が返る
        let id4 = id_allocator.allocate();
        assert!(id4 == id1 || id4 == 3);

        // 再度IDを発行 -> リリースされたid1が返る
        let id5 = id_allocator.allocate();
        assert!(id5 == id1 || id5 == 3);

        // 再度IDを発行 -> next_idから発行する
        let id6 = id_allocator.allocate();
        assert_eq!(id6, 4);
    }

    #[test]
    fn allocates_correct_numbers_when_multithreaded() {
        let id_allocator = Arc::new(IdAllocator::new());

        // 新規IDを発行
        let _ = id_allocator.allocate(); // -> 0
        let _ = id_allocator.allocate(); // -> 1
        let _ = id_allocator.allocate(); // -> 2
        let _ = id_allocator.allocate(); // -> 3
        let _ = id_allocator.allocate(); // -> 4
        let _ = id_allocator.allocate(); // -> 5

        id_allocator.release(1);
        id_allocator.release(3);

        let mut ids = Vec::new();

        for _ in 0..4 {
            let idalloc = Arc::clone(&id_allocator);
            let handle = thread::spawn(move || idalloc.allocate());
            ids.push(handle.join().unwrap());
        }

        // releaseしたIDが取り出されているか確認
        assert!(ids.contains(&1) || ids.contains(&3));

        // すでに発行済みかつreleaseしていないIDは取り出されていないことを確認
        assert!(ids.contains(&0) == false);
        assert!(ids.contains(&2) == false);
        assert!(ids.contains(&4) == false);
        assert!(ids.contains(&5) == false);

        // 発行されたIDを確認
        assert!(ids.contains(&6) == true);
        assert!(ids.contains(&7) == true);

        // 発行されていないことを確認
        assert!(ids.contains(&8) == false);
    }
}

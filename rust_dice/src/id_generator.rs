use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

///Allocates integer IDs and reuses freed IDs in ascending order.
pub struct IdGenerator {
    next_id: usize,
    free_ids: BinaryHeap<Reverse<usize>>,
    free_set: HashSet<usize>,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            free_ids: BinaryHeap::new(),
            free_set: HashSet::new(),
        }
    }

    pub fn allocate(&mut self) -> usize {
        if let Some(Reverse(id)) = self.free_ids.pop() {
            self.free_set.remove(&id);
            id
        } else {
            let return_id = self.next_id;
            self.next_id += 1;
            return_id
        }
    }

    pub fn free(&mut self, id: usize) {
        if id < self.next_id && self.free_set.insert(id) {
            self.free_ids.push(Reverse(id));
        }
    }
}
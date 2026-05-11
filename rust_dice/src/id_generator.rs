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

    pub fn free_all(&mut self){
        self.next_id = 0;
        self.free_ids.clear();
        self.free_set.clear();
    }
}

#[cfg(test)]
#[test]
fn test_allocate(){
    let mut id_gen = IdGenerator::new();

    let id_one = id_gen.allocate();
    println!("ID one = {}", id_one);

    let id_two = id_gen.allocate();
    println!("ID two = {}", id_two);

    let id_three = id_gen.allocate();
    println!("ID three = {}", id_three);

    id_gen.free(id_two);

    let id_four = id_gen.allocate();
    println!("ID four = {}", id_four);

    id_gen.free(id_one);

    let id_five = id_gen.allocate();
    println!("ID five = {}", id_five);

    id_gen.free_all();

    let id_six = id_gen.allocate();
    println!("ID six = {}", id_six);

    let id_seven = id_gen.allocate();
    println!("ID seven = {}", id_seven);

    id_gen.free(id_seven);

    let id_eight = id_gen.allocate();
    println!("ID eight = {}", id_eight);
}

#[test]
fn test_mass_allocate(){
    let mut id_gen = IdGenerator::new();

    for _ in 0..=100 {
        let id = id_gen.allocate();
        println!("ID = {}", id);
    }

    println!("Freeing 10 through 50");
    for i in 10..=50{
        id_gen.free(i);
    }

    for _ in 0..100{
        let id = id_gen.allocate();
        println!("ID = {}", id)
    }
}


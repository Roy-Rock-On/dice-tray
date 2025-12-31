use crate::dice::Die;
use crate::dice_data::{DieData, TypedDieData};
use crate::tray::Tray;
use crate::dice_profile::{DieProfile};

pub trait DiceAllocator{
    ///Creates a new die from a die profile, assigning it an id, and returning it as a Box<dyn Die>
    fn new_die(&mut self, profile : &DieProfile) -> Box<dyn Die>;

    ///Creates a new die from provided dice data. used when loading dice from a save file.
    fn new_die_from_data(&mut self, data : TypedDieData) -> Box<dyn Die>; 

    ///Creates a new dice tray with the given label and a unique ID.
    fn new_tray(&mut self, label: String) -> Box<dyn Tray>;
}

//Id generator manages ids with internal mutability.
pub struct IdGenerator{
    next_die_id : usize,
    next_tray_id : usize
}

impl IdGenerator{
    pub fn new() -> Self{
        IdGenerator { 
            next_die_id: 0,
            next_tray_id : 0 
        }
    }

    pub fn get_die_id(&mut self) -> usize {
        let next_id = self.next_die_id;
        self.next_die_id += 1;
        next_id
    }

    pub fn get_tray_id(&mut self) -> usize {
        let next_id = self.next_tray_id;
        self.next_tray_id += 1;
        next_id
    }
}
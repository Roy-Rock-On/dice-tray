use std::usize;

use crate::dice::{Die, Die32};
use crate::dice_data::{TypedDieData};
use crate::tray::Tray;
use crate::dice_profile::{DieProfile, DieProfileType};

pub trait DiceAllocator{
    ///Creates a new die from a die profile, assigning it an id, and returning it as a Box<dyn Die>
    fn new_die(&mut self, profile : &DieProfile) -> Result<Box<dyn Die>, String>;

    ///Creates a new die from provided dice data. used when loading dice from a save file.
    fn new_die_from_data(&mut self, data : TypedDieData) -> Result<Box<dyn Die>, String>; 

    ///Creates a new dice tray with the given label and a unique ID.
    fn new_tray(&mut self, label: String) -> Box<dyn Tray>;
}

//Id generator manages ids with internal mutability.
pub struct DieIdGenerator{
    next_die_id : usize,
}

impl DieIdGenerator{
    pub fn new() -> Self{
        DieIdGenerator { 
            next_die_id: 0,
        }
    }

    ///Maybe refactor this to use proper errors when I'm reay to learn mroe about error handling.
    pub fn get_die_id(&mut self) -> Result<usize, String> {
        if self.next_die_id == usize::MAX{
            return Err("Maximum die id reached, are you sure you need this many dice?".to_string());
        }

        let next_id = self.next_die_id;
        self.next_die_id += 1;
        Ok(next_id)
    }
}

///Creates a new die from a die profile.
pub fn new_die(id: usize, profile : &DieProfile) -> impl Die + use<> {
    match profile.die_type {
        DieProfileType::Numerical(faces) =>{
            Die32::new(id, profile.label.clone(), faces, profile.result_type)
        },
        DieProfileType::Custom => {
            todo!("Implement custom dice later.")
        }
    }
}

use rust_dice::tray::{Tray};
use rust_dice::dice_allocator::{DiceAllocator, IdGenerator};
use rust_dice::dice_builders::new_die;
use rust_dice::dice_profile::DieProfile;
use rust_dice::dice_data::{TypedDieData};
use rust_dice::dice::Die;
use crate::cli_dice_tray::{CliTray};

pub struct CliDiceAllocator{
    id_gen:  IdGenerator
}

impl DiceAllocator for CliDiceAllocator{
    fn new_die(&mut self, profile : &DieProfile) -> Box<dyn Die> {
        let new_die = new_die(self.id_gen.get_die_id(), profile);
        Box::new(new_die)
    }

    fn new_die_from_data(&mut self, data : TypedDieData) -> Box<dyn Die> {
        let new_die = data.to_die(self.id_gen.get_die_id());
        new_die
    }

    fn new_tray(&mut self, label : String) -> Box<dyn Tray>{
        let new_tray = CliTray::new(self.id_gen.get_tray_id(), label);
        Box::new(new_tray)
    }
}

impl CliDiceAllocator{
    ///Creates a new CLI Dice Allocator.
    pub fn new() -> Self{
        Self { 
            id_gen : IdGenerator::new()
        }
    }
}






use rust_dice::tray::{Tray};
use rust_dice::dice_allocator::{DiceAllocator, DieIdGenerator, new_die};
use rust_dice::dice_profile::DieProfile;
use rust_dice::dice_data::{TypedDieData};
use rust_dice::dice::Die;
use crate::cli_dice_tray::{CliTray};

pub struct CliDiceAllocator{
    id_gen: DieIdGenerator
}

impl DiceAllocator for CliDiceAllocator{
    fn new_die(&mut self, profile : &DieProfile) -> Result<Box<dyn Die>, String> {
        let new_die = new_die(self.id_gen.get_die_id()?, profile);
        Ok(Box::new(new_die))
    }

    fn new_die_from_data(&mut self, data : TypedDieData) -> Result<Box<dyn Die>, String> {
        let new_die = data.to_die(self.id_gen.get_die_id()?);
        Ok(new_die)
    }

    fn new_tray(&mut self, id : String) -> Box<dyn Tray>{
        let new_tray = CliTray::new(id);
        Box::new(new_tray)
    }
}

impl CliDiceAllocator{
    ///Creates a new CLI Dice Allocator.
    pub fn new() -> Self{
        Self { 
            id_gen : DieIdGenerator::new()
        }
    }
}





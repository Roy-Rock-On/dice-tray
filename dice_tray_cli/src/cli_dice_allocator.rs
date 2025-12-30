
use rust_dice::tray::Tray;
use rust_dice::dice_allocator::DiceAllocator;
use rust_dice::dice_builders::new_die;

pub struct CliDiceAllocator{
    dice_trays : Vec<Box<dyn Tray>>,
    next_die_id : usize,
}

impl DiceAllocator for CliDiceAllocator{
    fn init(&mut self){
        todo!("implement init.")
    }

    fn new_die(&mut self, profile : &rust_dice::dice_profile::DieProfile) -> Box<dyn rust_dice::dice::Die> {
        let new_die = new_die(self.next_die_id, profile);
        self.next_die_id += 1;
        Box::new(new_die)
    }

    fn close(&mut self) {
        todo!("implement close.")
    }
}

impl CliDiceAllocator{
    ///Creates a new CLI Dice Allocator.
    pub fn new() -> Self{
        Self { 
            dice_trays: Vec::new(),
            next_die_id: 0 
        }
    }
}
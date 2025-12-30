use crate::dice::Die;
use crate::dice_profile::{DieProfile};


pub trait DiceAllocator{
    //used to intitilize the dice allocator. Settings can be loaded here and dice in the tray can be checked and IDs can be assinged
    fn init(&mut self);

    ///Creates a new die from a die profile, assigning it an id, and rerunting it as a Box<dyn Die>
    fn new_die(&mut self, profile : &DieProfile) -> Box<dyn Die>; 

    ///Called when wrapping up the program. Used to save settings and similar.
    fn close(&mut self);
}
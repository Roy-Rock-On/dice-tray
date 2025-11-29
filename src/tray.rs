use super::dice::Die;

pub struct Tray {
    dice: Vec<Die>
}

impl Tray{
    /// Creates a new, empty Tray.
    pub fn new() -> Self {
        Tray {
            dice: Vec::new()
        }
    }

    /// Adds a Die to the tray.
    pub fn add_die(&mut self, die: Die) {
        self.dice.push(die);
    }

    /// Adds multiple Dice to the tray.
    pub fn add_dice(&mut self, dice: Vec<Die>) {
        for die in dice {
            self.dice.push(die);
        }
    }

    /// Removes a Die at the specified index from the tray.
    pub fn remove_at(&mut self, index: usize) -> Option<Die> {
        if index < self.dice.len() {
            Some(self.dice.remove(index)) 
        } else {
            None
        }
    }

    /// Rolls all Dice in the tray.
    pub fn roll_all(&mut self) {
        for die in &mut self.dice {
            die.roll();
        }
    }

    /// Rolls the Die at the specified index in the tray.
    pub fn roll_at(&mut self, index: usize) -> Option<u32> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll();
            Some(die.get_current_face())
        } else {
            None
        }
    }

    /// Clears all Dice from the tray.
    pub fn clear(&mut self) {
        self.dice.clear();
    }

    /// Returns a reference to the Dice in the tray.
    pub fn get_dice(&self) -> &Vec<Die> {
        &self.dice
    }
}
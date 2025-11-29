use super::dice::Die;

pub struct Tray {
    dice: Vec<Die>
}

impl Tray{
    pub fn new() -> Self {
        Tray {
            dice: Vec::new()
        }
    }

    pub fn add_die(&mut self, die: Die) {
        self.dice.push(die);
    }

    pub fn remove_at(&mut self, index: usize) -> Option<Die> {
        if index < self.dice.len() {
            Some(self.dice.remove(index)) 
        } else {
            None
        }
    }

    pub fn roll_all(&mut self) {
        for die in &mut self.dice {
            die.roll();
        }
    }

    pub fn roll_at(&mut self, index: usize) -> Option<u32> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll();
            Some(die.get_current_face())
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.dice.clear();
    }

    pub fn get_dice(&self) -> &Vec<Die> {
        &self.dice
    }
}
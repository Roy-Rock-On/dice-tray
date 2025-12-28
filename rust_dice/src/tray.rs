use super::dice::{Die, DieResultType};

pub struct Tray {
    next_id : usize,
    dice: Vec<Box<dyn Die>>, 
    tray_result_type: TrayResultType,
}

pub enum TrayResultType {
    Sum,
    Best,
    Worst,
}

impl TrayResultType {
    pub fn to_string(&self) -> String {
        match self {
            TrayResultType::Sum => "Tray sum".to_string(),
            TrayResultType::Best => "High roll in tray".to_string(),
            TrayResultType::Worst => "Worst roll in tray".to_string(),
        }
    }
}

pub enum TrayResult {
    Number(u32),
    String(String),
    None,
}

impl TrayResult {
    pub fn to_string(&self) -> String {
        match self {
            TrayResult::Number(n) => n.to_string(),
            TrayResult::String(s) => s.clone(),
            TrayResult::None => "None".to_string(),
        }
    }
}

impl Tray {
    /// Creates a new, empty Tray.
    pub fn new() -> Self {
        Tray {
            next_id : 0,
            dice: Vec::new(),
            tray_result_type: TrayResultType::Sum, // default result type
        }
    }

    /// This will have the be replaced by a better system to manage die IDs. But this might work fo now.
    pub fn get_next_die_id(&mut self) -> usize {
        let die_id = self.next_id;
        self.next_id += 1;
        die_id
    }

    /// Adds a Die to the tray.
    pub fn add_die(&mut self, die: Box<dyn Die>) {
        self.dice.push(die);
    }

    /// Adds multiple Dice to the tray.
    pub fn add_dice(&mut self, dice: Vec<Box<dyn Die>>) {
        for die in dice {
            self.dice.push(die);
        }
    }

    /// Removes a Die at the specified index from the tray.
    pub fn remove_at(&mut self, index: usize) -> Option<Box<dyn Die>> {
        if index < self.dice.len() {
            Some(self.dice.remove(index))
        } else {
            None
        }
    }

    /// Removes all Dice with the specified label from the tray. Returns all Dice removed.
    pub fn remove_by_label(&mut self, label: &str) -> Vec<Box<dyn Die>> {
        let mut removed_dice: Vec<Box<dyn Die>> = Vec::new();
        let mut i = 0;
        
        while i < self.dice.len() {
            if self.dice[i].get_label() == label {
                removed_dice.push(self.dice.remove(i));
            } else {
                i += 1;
            }
        }
        
        removed_dice
    }

    /// Rolls all Dice in the tray.
    pub fn roll_all(&mut self, result_type: DieResultType) {
        for die in &mut self.dice {
            die.roll(result_type);
        }
    }

    /// Rolls the Die at the specified index in the tray.
    pub fn roll_at(&mut self, index: usize, result_type: DieResultType) -> Result<(), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll(result_type);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Rolls all Dice in the tray with the specified label
    pub fn roll_by_label(&mut self, label: &str, result_type: DieResultType) -> Result<(), String> {
        let mut hit: bool = false;
        for die in self.dice.iter_mut() {
            if label == die.get_label() {
                die.roll(result_type);
                hit = true;
            }
        }
        if hit {
            Ok(())
        } else {
            Err("No dice with the specified identity found".to_string())
        }
    }

    /// Clears all Dice from the tray.
    pub fn clear(&mut self) {
        self.dice.clear();
    }

    /// Returns a reference to the Dice in the tray.
    pub fn get_dice(&self) -> &Vec<Box<dyn Die>> {
        &self.dice
    }

    pub fn get_tray_result_type(&self) -> &TrayResultType {
        &self.tray_result_type
    }

    pub fn get_tray_result(&self) -> TrayResult {
        match self.tray_result_type {
            TrayResultType::Sum => {
                let sum: u32 = self
                    .dice
                    .iter()
                    .map(|die| die.get_result().is_num_or(0))
                    .sum();
                TrayResult::Number(sum)
            }
            TrayResultType::Best => {
                let best = self
                    .dice
                    .iter()
                    .map(|die| die.get_result().is_num_or(0))
                    .max();
                match best {
                    Some(value) => TrayResult::Number(value),
                    None => TrayResult::None,
                }
            }
            TrayResultType::Worst => {
                let worst = self
                    .dice
                    .iter()
                    .map(|die| die.get_result().is_num_or(0))
                    .min();
                match worst {
                    Some(value) => TrayResult::Number(value),
                    None => TrayResult::None,
                }
            }
        }
    }
}

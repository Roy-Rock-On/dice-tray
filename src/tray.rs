use super::dice::Die;

pub struct Tray {
    dice: Vec<Die>,
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

pub enum TrayResult{
    Number(u32),
    String(String),
    None
}

impl TrayResult {
    pub fn to_string(&self) -> String {
        match self {
            TrayResult::Number(n) =>  n.to_string(),
            TrayResult::String(s) => s.clone(),
            TrayResult::None => "None".to_string(),
        }
    }
}


impl Tray{
    /// Creates a new, empty Tray.
    pub fn new() -> Self {
        Tray {
            dice: Vec::new(),
            tray_result_type: TrayResultType::Sum, // default result type
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

    /// Removes all Dice with the specified identity from the tray.
    pub fn remove_by_id(&mut self, identity: &str) -> Vec<Die> {
        let mut removed_dice: Vec<Die> = Vec::new();
        self.dice.retain(|die| {
            if die.get_id() == identity {
                removed_dice.push(die.clone());
                false
            } else {
                true
            }
        });
        removed_dice
    }

    /// Sets the identity of the Die at the specified index in the tray.
    pub fn set_identity_at(&mut self, index: usize, identity: String) -> Result<(String, String), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            let old_id = die.get_id().to_string();
            die.set_identity(identity);
            let new_id = die.get_id().to_string();
            Ok((old_id, new_id))
        } else {
            Err("Cannot set dice identity as provided index is out of bounds".to_string())
        }
    }

    /// Rolls all Dice in the tray.
    pub fn roll_all(&mut self) {
        for die in &mut self.dice {
            die.roll();
        }
    }

    /// Rolls the Die at the specified index in the tray.
    pub fn roll_at(&mut self, index: usize) -> Result<(), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll();
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Rolls all Dice in the tray with the specified identity
    pub fn roll_by_id(&mut self, identity: &str) -> Result<(), String> {
        let mut hit : bool = false;
        for die in self.dice.iter_mut() {
                if identity == die.get_id() {
                    die.roll();
                    hit = true;
                }
            }
        if hit {
            Ok(())
        } else {
            Err("No dice with the specified identity found".to_string())
        }
    }

    pub fn roll_at_identities(&mut self, identity: &str, indices: &Vec<usize>) -> Result<(), String> {
        let mut hit : bool = false;
        for (i, die) in self.dice.iter_mut().enumerate() {
                if identity == die.get_id() && indices.contains(&i) {
                    die.roll();
                    hit = true;
                }
            }
        if hit {
            Ok(())
        } else {
            Err("No dice with the specified identity found at the specified indices".to_string())
        }
    }

    /// Sets the result type of the Die with the specified identity in the tray.
    pub fn set_result_type_at(&mut self, index: usize, result_type: super::dice::DieResultType) -> Result<(), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.set_result_type(result_type);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Sets the result type of the Die with the specified identity in the tray.
    pub fn set_result_type_by_id(&mut self, identity: &str, result_type: super::dice::DieResultType) -> Result<(), String> {
        let mut hit : bool = false;
        for die in self.dice.iter_mut() {
                if identity == die.get_id() {
                    die.set_result_type(result_type.clone());
                    hit = true;
                }
            }
        if hit {
            Ok(())
        } else {
            Err("No dice with the specified identity found".to_string())
        }
    }

    /// Sets the result type of the Die at the specified index in the tray.
    pub fn set_all_result_type(&mut self, result_type: super::dice::DieResultType) {
        for die in &mut self.dice {
            die.set_result_type(result_type.clone());
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

    pub fn get_tray_result_type(&self) -> &TrayResultType {
        &self.tray_result_type
    }

    pub fn get_tray_result(&self) -> TrayResult {
        match self.tray_result_type {
            TrayResultType::Sum => {
                let sum: u32 = self.dice.iter().map(|die| die.get_result_value().unwrap_or(0)).sum();
                TrayResult::Number(sum)
            },
            TrayResultType::Best => {
                let best = self.dice.iter().map(|die| die.get_result_value().unwrap_or(0)).max();
                match best {
                    Some(value) => TrayResult::Number(value),
                    None => TrayResult::None,
                }
            },
            TrayResultType::Worst => {
                let worst = self.dice.iter().map(|die| die.get_result_value().unwrap_or(0)).min();
                match worst {
                    Some(value) => TrayResult::Number(value),
                    None => TrayResult::None,
                }
            },
        }
    }
}
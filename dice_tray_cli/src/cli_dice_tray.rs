use rust_dice::dice::{Die, DieResultType};
use rust_dice::dice_data::{DieData, DieData32, TypedDieData};
use rust_dice::tray::{Tray, TrayResult, TrayResultType};

use std::fmt::Write;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CliTrayData {
    label: String,
    dice_data: Vec<TypedDieData>,
}

impl From<&dyn Tray> for CliTrayData {
    fn from(tray: &dyn Tray) -> Self {
        let tray_dice = tray.get_dice();
        let dice_data: Vec<TypedDieData> = tray_dice
            .iter()
            .map(|die| <DieData32 as DieData>::from_die(die.as_ref()))
            .collect();

        CliTrayData {
            label: tray.get_id().to_string(),
            dice_data,
        }
    }
}

impl CliTrayData {
    /// Get the label of this tray data
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Get the dice data vector
    pub fn get_dice_data(&self) -> Vec<TypedDieData> {
        self.dice_data.clone()
    }
}

pub struct CliTray {
    id: String,
    dice: Vec<Box<dyn Die>>,
    tray_result_type: TrayResultType,
}

impl CliTray {
    /// Creates a new, empty Tray.
    pub fn new(id: String) -> Self {
        CliTray {
            id,
            dice: Vec::new(),
            tray_result_type: TrayResultType::Sum, // default result type
        }
    }
}

impl Tray for CliTray {
    /// trays have unique ID's that are used by the dice allocator to create a HashSet. The allocator ensure tha there are no duplicate tray IDs in the application.  
    fn get_id(&self) -> &str {
        &self.id
    }

    /// Adds a Die to the tray.
    fn add_die(&mut self, die: Box<dyn Die>) {
        self.dice.push(die);
    }

    /// Adds multiple Dice to the tray.
    fn add_dice(&mut self, dice: Vec<Box<dyn Die>>) {
        for die in dice {
            self.dice.push(die);
        }
    }

    /// Removes a Die at the specified index from the tray, or returns an error if no dice is found at the index.
    fn remove_die_at(&mut self, index: usize) -> Result<Box<dyn Die>, String> {
        if index < self.dice.len() {
            Ok(self.dice.remove(index))
        } else {
            Err(format![
                "No dice found at provided index: {}. No dice removed from tray.",
                index
            ])
        }
    }

    ///Removes a single die from the tray that matches the provided ID.
    ///Returns only the first die found. There shouldn't be multipule dice with the same ID active in the app if the DiceAllocator is being used properly.  
    fn remove_die_by_id(&mut self, id: usize) -> Result<Box<dyn Die>, String> {
        let mut i = self.dice.len();
        loop {
            if self.dice[i].get_id() == id {
                return Ok(self.dice.remove(i));
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        return Err(format![
            "No die with ID: {} found in tray. No die has been removed.",
            id
        ]);
    }

    /// Removes all Dice with the specified label from the tray. Returns all Dice removed.
    fn remove_dice_by_label(&mut self, label: &str) -> Result<Vec<Box<dyn Die>>, String> {
        let mut removed_dice: Vec<Box<dyn Die>> = Vec::new();
        let mut i = self.dice.len() - 1;

        loop {
            if self.dice[i].get_label() == label {
                removed_dice.push(self.dice.remove(i));
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if removed_dice.is_empty() {
            return Err("No dice with the specified label found".to_string());
        }
        Ok(removed_dice)
    }

    /// Rolls all Dice in the tray.
    fn roll_all(&mut self, result_type: Option<DieResultType>) {
        for die in &mut self.dice {
            die.roll(result_type);
        }
    }

    /// Rolls the Die at the specified index in the tray.
    fn roll_at(&mut self, index: usize, result_type: Option<DieResultType>) -> Result<(), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll(result_type);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Rolls all Dice in the tray with the specified label
    fn roll_by_label(
        &mut self,
        label: &str,
        result_type: Option<DieResultType>,
    ) -> Result<(), String> {
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

    fn sort(&mut self, _sort_by: rust_dice::tray::TraySortType) {
        todo!("Must implement sort for cli_dice_tray.");
    }

    fn remove_all(&mut self) -> Result<Vec<Box<dyn Die>>, String> {
        let dice: Vec<Box<dyn Die>> = self.dice.drain(..).collect();
        if dice.is_empty() {
            return Err(format![
                "Tray is already empty, no dice returned by remove_all."
            ]);
        }
        Ok(dice)
    }

    /// Clears all Dice from the tray.
    fn clear(&mut self) {
        self.dice.clear();
    }

    /// Returns a reference to the Dice in the tray.
    fn get_dice(&self) -> &Vec<Box<dyn Die>> {
        &self.dice
    }

    fn get_result_type(&self) -> &TrayResultType {
        &self.tray_result_type
    }

    fn get_result(&self) -> TrayResult {
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

    fn get_summary(&self) -> String {
        let mut summary_string = String::new();
        for die in self.dice.iter().enumerate(){
            write!(summary_string, "@{}:{}", die.0, die.1.get_summary()).unwrap()
        }   
        summary_string
    }
}
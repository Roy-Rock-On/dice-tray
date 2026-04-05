use rust_dice::{dice::{Die, Die32}, dice_data::{DieData, TypedDieData, DieData32}};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

pub struct WasmTray{
    id: usize,
    label: String,
    dice: Vec<Die32>,
    next_die_id: usize
}

impl WasmTray{
    pub fn new(id: usize) -> Self{
        WasmTray{
            id,
            label: format!("New Tray"),
            dice: Vec::new(),
            next_die_id: 0
        }
    }
    
     /// trays have unique ID's that are used by the dice allocator to create a HashSet. The allocator ensure tha there are no duplicate tray IDs in the application.  
    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Adds a Die to the tray.
    pub fn add_die(&mut self, die: Die32) {
        self.dice.push(die);
    }

    /// Removes a Die at the specified index from the tray, or returns an error if no dice is found at the index.
    pub fn remove_die_at(&mut self, index: usize) -> Result<Die32, String> {
        if index < self.dice.len() {
            Ok(self.dice.remove(index))
        } else {
            Err(format![
                "No dice found at provided index: {}. No dice removed from tray.",
                index
            ])
        }
    }

    /// Rolls all Dice in the tray.
    pub fn roll_all(&mut self) {
        for die in &mut self.dice {
            die.roll(Some(*die.get_result_type()));
        }
    }

    /// Rolls the Die at the specified index in the tray.
    pub fn roll_at(&mut self, index: usize) -> Result<(), String> {
        if index < self.dice.len() {
            let die = &mut self.dice[index];
            die.roll(Some(*die.get_result_type()));
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Clears all Dice from the tray.
    pub fn clear(&mut self) {
        self.dice.clear();
    }

    ///gets the next die id as needed.
    pub fn get_next_die_id(&mut self) -> usize{
        let ret_id = self.next_die_id;
        self.next_die_id += 1;
        ret_id
    }
}


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct TrayData{
    id: usize,
    label: String,
    dice: Vec<TypedDieData>,
}

#[wasm_bindgen]
impl TrayData {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> String {
        self.label.clone()
    }

    // For the dice array, you'll need to handle this carefully
    #[wasm_bindgen(getter)]
    pub fn dice_count(&self) -> usize {
        self.dice.len()
    }

    // Method to get dice data as JSON string (simpler approach)
    #[wasm_bindgen]
    pub fn get_dice_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.dice)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl TrayData{
    pub fn from_tray(tray: &WasmTray) -> TrayData{
        let dice_data: Vec<TypedDieData> = tray.dice.iter().map(
            |d| <DieData32 as DieData>::from_die(d)).collect();
        
        TrayData{
            id: *tray.get_id(),
            label: tray.get_label().to_string(),
            dice: dice_data,            
        }
    }
}
use dirs::{data_local_dir};

use rust_dice::dice::{Die};
use rust_dice::dice_allocator::DiceAllocator;
use rust_dice::tray::{Tray};
use rust_dice::dice_data::{DieData, DieData32, TypedDieData};

use std::fs::create_dir_all;
use std::error::{Error};

use crate::cli_dice_allocator::{CliDiceAllocator};
use crate::cli_dice_tray::{CliTray, CliTrayData};

pub struct CliDiceTrayApp{
    dice_allocator : CliDiceAllocator,
    dice_trays : Vec<Box<dyn Tray>>
}

impl CliDiceTrayApp{
    pub fn new() -> Self{
        CliDiceTrayApp{
            dice_allocator : CliDiceAllocator::new(),
            dice_trays : Vec::new()
        }
    }

    pub fn init(&mut self){
        if let Err(e) = self.load_trays_from_file() {
            println!("Error loading trays from file: {}", e);
        } else {
            println!("Welcome to dice-tray cli. Your trays have loaded successfully!");
        }
    }

    pub fn close(&mut self){
        if let Err(e) = self.save_trays_to_file() {
            println!("Error saving trays to file: {}", e);
        } else {
            println!("Trays saved successfully. Goodbye!");
        }
    }

    fn load_trays_from_file(&mut self) -> Result<Vec<Box<dyn Tray>>, Box<dyn Error>> {
        let data_dir = match data_local_dir() {
            Some(dir) => dir.join("dice-tray"),
            None => {
                eprintln!("Warning: Could not get local data directory, using current directory");
                std::env::current_dir()?.join("dice-tray")
            }
        };
        
        if !data_dir.exists() {
            create_dir_all(&data_dir)?;
        }
        
        let save_file = data_dir.join("dice_tray_save.json");
        let file_content = std::fs::read_to_string(&save_file)?;
        let tray_data_vec: Vec<CliTrayData> = serde_json::from_str(&file_content)?;
        
        let mut loaded_trays: Vec<Box<dyn Tray>> = Vec::new();
        for data in tray_data_vec {
            let dice_data = data.get_dice_data();
            let mut tray = self.dice_allocator.new_tray(data.get_label().to_string());
            let mut tray_dice: Vec<Box<dyn Die>> = Vec::new();
            for datum in dice_data {
                tray_dice.push(self.dice_allocator.new_die_from_data(datum));
            }
            tray.add_dice(tray_dice);
            loaded_trays.push(tray);
        }
        
        Ok(loaded_trays)
    }

        fn save_trays_to_file(&self) -> Result<(), Box<dyn Error>> {
            let mut tray_data_vec: Vec<CliTrayData> = Vec::new();
            
            for tray in &self.dice_trays {
                let tray_data = CliTrayData::from(tray.as_ref());
                tray_data_vec.push(tray_data);
            }
            
            let data_dir = data_local_dir()
                .ok_or("Failed to get local data directory")?
                .join("dice-tray");
            
            if !data_dir.exists() {
                create_dir_all(&data_dir)?;
            }
            
            let save_file = data_dir.join("dice_tray_save.json");
            let json_content = serde_json::to_string_pretty(&tray_data_vec)?;
            std::fs::write(&save_file, json_content)?;
            
            Ok(())
        }
}
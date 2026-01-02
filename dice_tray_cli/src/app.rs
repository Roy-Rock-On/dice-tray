use dirs::{data_local_dir};

use rust_dice::dice::{Die, DieResultType};
use rust_dice::dice_allocator::DiceAllocator;
use rust_dice::dice_profile::{DieProfile, DieProfileType};
use rust_dice::tray::{Tray};

use indexmap::IndexMap;

use std::fs::create_dir_all;
use std::error::{Error};

use crate::cli_dice_allocator::{CliDiceAllocator};
use crate::cli_dice_tray::{CliTrayData};
use crate::cli_parser::DiceTargets;
use crate::logger::detailed_log_tray;

pub struct CliDiceTrayApp{
    dice_allocator : CliDiceAllocator,
    dice_trays : IndexMap<String, Box<dyn Tray>>,
}

impl CliDiceTrayApp{
    pub fn new() -> Self{
        CliDiceTrayApp{
            dice_allocator : CliDiceAllocator::new(),
            dice_trays : IndexMap::new(),
        }
    }

    pub fn init(&mut self){
        println!("Welcome to dice-tray cli.");
        match self.load_trays_from_file(){
            Ok(trays) => {
                let mut tray_duplicate = false;
                for tray in trays.into_iter(){
                    if !self.dice_trays.contains_key(tray.get_id()){
                        self.dice_trays.insert(tray.get_id().to_string(), tray);
                    }
                    else{
                        println!("A tray with ID: {} has already been loaded. Tray with duplicate ID cannot be loaded.", tray.get_id());
                        tray_duplicate = true;
                    }
                }
                if (tray_duplicate){
                    println!("Some trays were not loaded due to errors. Sorry about that.");
                }
                else{
                    println!("Your trays have loaded successfully!");
                }
            },
            Err(e) => {
                println!("Error loading trays from file: {}", e);
            }
        }

        if self.dice_trays.is_empty(){
            println!("No trays loaded. You can create a new tray by targeting it.");
        }
    }

    pub fn close(&mut self){
        if let Err(e) = self.save_trays_to_file() {
            println!("Error saving trays to file: {}", e);
        } else {
            println!("Trays saved successfully. Goodbye!");
        }
    }

    pub fn add_dice_from_raw(&mut self, tray_id : Option<&str>, count : u32, faces : u32){
        let profile = DieProfile::new(None, DieProfileType::Numerical(faces));
        for _i in 0..count{
            match self.dice_allocator.new_die(&profile){
                Ok(die) => {
                    match self.get_tray_mut(tray_id){
                        Ok(tray) => tray.add_die(die),
                        Err(e) => println!("Failed to add dice with error {}", e)
                    }
                }
,
                Err(e) => {
                    println!("Failed to add new dice to tray from raw. Error: {}", e);
                }
            }
        }
    }

    pub fn show_tray(&self, tray_id : Option<&str>){
        match tray_id {
            Some(id) => {
                match self.dice_trays.get(id){
                    Some(tray) => detailed_log_tray(tray.as_ref()),
                    None => { println!("No tray found with id = {} to show.", id)}
                };
            }
            None => {
                match self.dice_trays.first(){
                    Some(tray) => detailed_log_tray(tray.1.as_ref()),  
                    None => println!("No Trays found at all. This shouldn't happen.")
                };
            }
        }
    }

    pub fn roll_all(&mut self, tray_id: Option<&str>){
        match self.get_tray_mut(tray_id) {
            Ok(active_tray) => active_tray.roll_all(rust_dice::dice::DieResultType::Face),
            Err(e) => println!("Roll all failed with error {}", e)
        }
    }

    pub fn roll_at_targets(&mut self, tray_id : Option<&str>, targets: Vec<DiceTargets>) -> Result<(), String>{
        if let Ok(active_tray) = self.get_tray_mut(tray_id){
            targets.iter().for_each(|target| {
                match target{
                    DiceTargets::Index(indecies) => {
                        for i in indecies.iter() { 
                            let _ = active_tray.roll_at(*i, DieResultType::Face);
                        }
                    },
                    DiceTargets::Label(label) =>{
                        match active_tray.roll_by_label(label, DieResultType::Face){
                            Ok(_) => {},
                            Err(_) => {}
                        }
                    }
                }
            });
        }
        Ok(())
    }

    pub fn remove_at_targets(&mut self, tray_id: Option<&str>, targets: Vec<DiceTargets>) -> Result<(), String>{
        if let Ok(active_tray) = self.get_tray_mut(tray_id){
            targets.iter().for_each(|target| {
                match target{
                    DiceTargets::Index(indices) => {
                        for i in indices.iter().rev() { 
                            let _ = active_tray.remove_die_at(*i);
                        }
                    },
                    DiceTargets::Label(label) =>{
                        match active_tray.remove_dice_by_label(label){
                            Ok(_) => {},
                            Err(_) => {}
                        }
                    }
                }
            });
        }
        Ok(())
    }

    ///Resets the whole application by clearning all trays and dice. 
    pub fn reset(&mut self){
        println!("Resetting dice-tray by deleting all trays and dice and trays.");
        self.dice_trays.clear(); 
    }
    
    fn get_tray(&mut self, id : Option<String>) -> Result<&dyn Tray, String>{
        match id {
            Some(key) => {
                if !self.dice_trays.contains_key(&key) {
                    let new_tray = self.dice_allocator.new_tray(key.clone());
                    self.dice_trays.insert(key.clone(), new_tray);
                    println!("Creating a new tray with id = {}", key);
                }
                Ok(self.dice_trays.get(&key).unwrap().as_ref())
            },
            None => {
                match self.dice_trays.get_index(0) {
                    Some((_, tray)) => Ok(tray.as_ref()),
                    None => Err(format!("No dice trays found at all. How!?"))
                }   
            }
        }
    }

    fn get_tray_mut(&mut self, id : Option<&str>) -> Result<&mut dyn Tray, String>{
        match id {
            Some(key) => {
                if !self.dice_trays.contains_key(key) {
                    let new_tray = self.dice_allocator.new_tray(key.to_string());
                    self.dice_trays.insert(key.to_string(), new_tray);
                    println!("Creating a new tray with id = {}", key);
                }
                Ok(self.dice_trays.get_mut(key).unwrap().as_mut())
            },
            None => {
                match self.dice_trays.get_index_mut(0) {
                    Some((_, tray)) => Ok(tray.as_mut()),
                    None => Err(format!("No dice trays found at all. How!?"))
                }   
            }
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
                tray_dice.push(self.dice_allocator.new_die_from_data(datum)?);
            }
            tray.add_dice(tray_dice);
            loaded_trays.push(tray);
        }
        
        Ok(loaded_trays)
    }

        fn save_trays_to_file(&mut self) -> Result<(), Box<dyn Error>> {
            let mut tray_data_vec: Vec<CliTrayData> = Vec::new();

            for tray in self.dice_trays.iter() {
                let tray_data = CliTrayData::from(tray.1.as_ref());
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
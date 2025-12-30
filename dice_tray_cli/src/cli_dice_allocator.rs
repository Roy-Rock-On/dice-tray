
use std::sync::LazyLock;
use rust_dice::{dice_profile::DieType, tray::Tray};
use rust_dice::dice_allocator::DiceAllocator;
use rust_dice::dice_builders::new_die;
use rust_dice::dice_profile::DieProfile;
use rust_dice::dice::Die;
use std::fs;
use std::path::Path;
use crate::cli_dice_tray::{CliTray, CliTrayData};
use std::env;

static DEFAULT_SAVE_PATH: LazyLock<std::path::PathBuf> = LazyLock::new(|| get_default_save_path());

fn get_default_save_path() -> std::path::PathBuf {
    let config_dir = if cfg!(target_os = "windows") {
        env::var("APPDATA").unwrap_or_else(|_| ".".to_string())
    } else {
        env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.config", home)
        })
    };
    
    let dice_tray_dir = Path::new(&config_dir).join("dice-tray");
    
    // Create the directory if it doesn't exist
    if !dice_tray_dir.exists() {
        if let Err(e) = fs::create_dir_all(&dice_tray_dir) {
            eprintln!("Warning: Could not create config directory: {}", e);
        }
    }
    
    dice_tray_dir.join("dice_tray_save.json")
}


pub struct CliDiceAllocator{
    dice_trays : Vec<Box<dyn Tray>>,
    id_gen:  IdGenerator
}

impl DiceAllocator for CliDiceAllocator{

    fn init(&mut self){
        // Try to load existing tray data from file
        if Path::new(&*DEFAULT_SAVE_PATH).exists() {
            match self.load_trays_from_file() {
                Ok(loaded_trays) => {
                    self.dice_trays = loaded_trays;
                    println!("Loaded {} trays from {}", self.dice_trays.len(), DEFAULT_SAVE_PATH.display());
                }
                Err(e) => {
                    eprintln!("Error loading trays from file: {}", e);
                }
            }
        } else {
            println!("No existing save file found. Starting fresh.");
        }

        if self.dice_trays.is_empty(){
            let new_tray = CliTray::new(self.id_gen.get_tray_id(), "die-tray".to_string());
            self.dice_trays.push(Box::new(new_tray));
        }
    }

    fn new_die(&mut self, profile : &DieProfile) -> Box<dyn Die> {
        let new_die = new_die(self.id_gen.get_die_id(), profile);
        Box::new(new_die)
    }

    fn new_tray(&mut self, label : String) -> Box<dyn Tray>{
        let new_tray = CliTray::new(self.id_gen.get_tray_id(), label);
        Box::new(new_tray)
    }

    fn close(&mut self) {
        // Convert current trays to data and save to file
        match self.save_trays_to_file() {
            Ok(_) => {
                println!("Successfully saved {} trays to {}", self.dice_trays.len(), DEFAULT_SAVE_PATH.display());
            }
            Err(e) => {
                eprintln!("Error saving trays to file: {}", e);
            }
        }
    }
}

impl CliDiceAllocator{
    ///Creates a new CLI Dice Allocator.
    pub fn new() -> Self{
        Self { 
            dice_trays: Vec::new(),
            id_gen : IdGenerator::new()
        }
    }

    pub fn get_tray_test(&self) -> &Box<dyn Tray>{
        &self.dice_trays[0]
    }

    pub fn test_add_dice(&mut self){
        let profile = DieProfile::new(None, DieType::Numerical(12));
        let new_die = self.new_die(&profile);
        self.dice_trays[0].add_die(new_die);
    }

    /// Load trays from the default file path
    fn load_trays_from_file(&mut self) -> Result<Vec<Box<dyn Tray>>, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(&*DEFAULT_SAVE_PATH)?;
        let tray_data_vec: Vec<CliTrayData> = serde_json::from_str(&file_content)?;
        
        // Convert CliTrayData back to CliTray instances
        // For now, we'll create empty trays with the saved IDs and labels
        let mut loaded_trays: Vec<Box<dyn Tray>> = Vec::new();
        for data in tray_data_vec {
            let dice_data = data.get_dice_data();
            let mut tray = CliTray::new(data.get_id(), data.get_label().to_string());
            let mut tray_dice: Vec<Box<dyn Die>> = Vec::new();
            for datum in dice_data {
                tray_dice.push(datum.to_die(self.id_gen.get_die_id()))
            }
            tray.add_dice(tray_dice);
            loaded_trays.push(Box::new(tray));
        }
        
        Ok(loaded_trays)
    }

    /// Save current trays to the default file path
    fn save_trays_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Convert all trays to TrayData
        let mut tray_data_vec: Vec<CliTrayData> = Vec::new();
        
        for tray in &self.dice_trays {
            // We need to work around the fact that from_tray takes ownership
            // For now, we'll create CliTrayData manually
            // TODO: Improve this when the TrayData trait is refined
            let tray_data = CliTrayData::from_tray_ref(tray.as_ref());
            tray_data_vec.push(tray_data);
        }
        
        let json_content = serde_json::to_string_pretty(&tray_data_vec)?;
        fs::write(&*DEFAULT_SAVE_PATH, json_content)?;
        
        Ok(())
    }
}

struct IdGenerator{
    next_die_id : usize,
    next_tray_id : usize
}

impl IdGenerator{
    fn new() -> Self{
        IdGenerator { 
            next_die_id: 0,
            next_tray_id : 0 
        }
    }

    fn get_die_id(&mut self) -> usize {
        let next_id = self.next_die_id;
        self.next_die_id += 1;
        next_id
    }

    fn get_tray_id(&mut self) -> usize {
        let next_id = self.next_tray_id;
        self.next_tray_id += 1;
        next_id
    }
}


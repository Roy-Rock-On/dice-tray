use std::collections::HashMap;
use std::sync::LazyLock;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use super::tables::{DiceResultTable, implement_test_table};

pub static DICE_TRAY_SETTINGS : LazyLock<DiceTraySettings> = LazyLock::new(|| {
    let mut settings = DiceTraySettings::new();
    settings.add_result_table("test".to_string(), implement_test_table());
    settings
});

#[derive(Serialize, Deserialize)]
pub struct DiceTraySettings {
    pub result_tables: HashMap<String, DiceResultTable>
}

impl DiceTraySettings {
    /// Creates a new DiceTraySettings with default settings.
    pub fn new() -> Self {
        DiceTraySettings {
            result_tables: HashMap::new()
        }
    }

    /// Adds a new result table to the settings.
    pub fn add_result_table(&mut self, name: String, table: DiceResultTable) {
        self.result_tables.insert(name, table);
    }

    pub fn dice_table_lookup(&self, table_name: &str, dice_result: u32) -> Result<&str, String> {
        match self.result_tables.get(table_name) {
            Some(table) => {
                match table.lookup(dice_result) {
                    Ok(result_string) => Ok(result_string),
                    Err(error) => Err(format!("No result found for dice result {} in table {} | Error {}", dice_result, table_name, error))
                }
            },
            None => Err(format!("No table found with name {}", table_name))
        }
    }

    pub fn has_table(&self, table_name: &str) -> bool {
        self.result_tables.contains_key(table_name)
    }

    /// Get the path to the settings file
    fn settings_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        
        let app_dir = config_dir.join("dice-tray");
        fs::create_dir_all(&app_dir).ok();
        
        app_dir.join("dice-tray-settings.json")
    }

    /// Load settings from disk, or create default if not found
    pub fn load(&self) -> Self {
        let path = Self::settings_path();
        
        if let Ok(contents) = fs::read_to_string(&path) {
            serde_json::from_str(&contents).unwrap_or_else(|_| {
                eprintln!("Failed to parse settings file, using defaults");
                Self::new()
            })
        } else {
            Self::new()
        }
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::settings_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|_| "Error serializing dice-tray-settings to JSON. Data has not been saved.".to_string())?;
        
        fs::write(path, json.as_bytes())
            .map_err(|_| "Error writing dice-tray-settings to configuration file. Data has not been saved.".to_string())
    }
}
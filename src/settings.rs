use std::collections::HashMap;
use crate::tables::implement_test_table;
use std::sync::LazyLock;

pub static DICE_TRAY_SETTINGS : LazyLock<DiceTraySettings> = LazyLock::new(|| {
    let mut settings = DiceTraySettings::new();
    settings.add_result_table("test".to_string(), implement_test_table().table);
    settings
});

pub struct DiceTraySettings {
    pub result_tables: HashMap<String, HashMap<u32, String>>
}

impl DiceTraySettings {
    /// Creates a new DiceTraySettings with default settings.
    pub fn new() -> Self {
        DiceTraySettings {
            result_tables: HashMap::new()
        }
    }

    /// Adds a new result table to the settings.
    pub fn add_result_table(&mut self, name: String, table: HashMap<u32, String>) {
        self.result_tables.insert(name, table);
    }

    pub fn dice_table_lookup(&self, table_name: &str, dice_result: u32) -> Result<String, String> {
        match self.result_tables.get(table_name) {
            Some(table) => {
                match table.get(&dice_result) {
                    Some(result_string) => Ok(result_string.clone()),
                    None => Err(format!("No result found for dice result {} in table {}", dice_result, table_name))
                }
            },
            None => Err(format!("No table found with name {}", table_name))
        }
    }

    pub fn has_table(&self, table_name: &str) -> bool {
        self.result_tables.contains_key(table_name)
    } 
}
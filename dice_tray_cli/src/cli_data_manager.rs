use std::str::FromStr;
use std::{fs, path};
use std::io::{Write};
use std::fs::{read_to_string};
use std::path::{Path, PathBuf};

use std::fs::create_dir_all;
use serde_json;
use rust_dice::die::DiceDataList;
use anyhow::{Error, bail};

pub struct DiceTrayDataManager{
    directory_path : PathBuf,
    filename : String,
}

impl DiceTrayDataManager{
    pub fn new() -> anyhow::Result<Self> {
        let mut data_dir = match dirs::data_dir(){
            Some(path) => path,
            None => bail!("Default data path not found.")
        };
        data_dir.push("dice-tray");

        if !data_dir.exists(){
            create_dir_all(&data_dir);
        }

        Ok(Self {
            directory_path: data_dir,
            filename: "default-dice-tray-data.json".to_string(),
        })
    }

        // Function to set a new file path
    pub fn set_filepath(&mut self, path_string: String) -> anyhow::Result<()> {
        let dir_path = PathBuf::from_str(&path_string)?;
        if dir_path.is_dir(){
            self.directory_path = dir_path;
            Ok(())
        }else{
            bail!("Cannot set new data path {} does not point to a valid directory.", path_string);
        }
    }

    pub fn get_directory_str(&self) -> &str {
        self.directory_path.to_str().unwrap()
    } 

    pub fn set_filename(&mut self, mut filename_string: String) -> anyhow::Result<()> {
        filename_string.push_str(".json");
        let full_path = self.directory_path.join(filename_string);
        
        if !full_path.is_file(){
            fs::File::create(&full_path)?;
        }

        Ok(())
    }

    pub fn get_filename_str(&self) -> &str{
        &self.filename
    }

    pub fn load_dice(&self) -> anyhow::Result<DiceDataList>{
        let file_path = self.directory_path.join(&self.filename);
        let die_file = read_to_string(&file_path)?;
        let decoded_list: DiceDataList = serde_json::from_str(&die_file)?;
        Ok(decoded_list)
    }

    pub fn save_dice(&mut self, dice_bag: &DiceDataList) -> Result<(), Error>{
        let file_path = self.directory_path.join(&self.filename);
        let die_list_json = serde_json::to_string_pretty(&dice_bag)?;

        let mut save_file = fs::File::create(&file_path)?;
        save_file.write_all(die_list_json.as_bytes())?;

        Ok(())
    }
}
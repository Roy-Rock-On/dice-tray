use std::{fs, path};
use std::io::Write;
use std::path::{Path, PathBuf};

use std::fs::create_dir_all;
use serde_json;
use rust_dice::die::DiceDataList;
use anyhow::{Error, bail};

pub struct DiceTrayDataManager{
    data_dir : Option<String>,
    filename : Option<String>,
    last_path : Option<PathBuf>
}

impl DiceTrayDataManager{
    pub fn load_dice(&mut self, dir_path : Option<String>, file_name : Option<String>) -> Result<DiceDataList, Error>{
        let path = match dir_path {
            Some(s) => {
                if !Path::new(&s).is_dir(){
                    bail!("Cannot load dice as {} is not a valid directory." , s);
                }
                let new_path = PathBuf::from(s);
                new_path
            },
            None => {
                match self.last_path {
                    Some(last) => last,
                    None =>{
                        let mut default_path = dirs::data_dir().unwrap();
                        default_path.push("dice-tray");
                        if !default_path.exists(){
                            create_dir_all(default_path);
                        }
                        default_path
                    }
                }

            }
        };

        if !path.is_dir(){

        }
        
        let rel = PathBuf::from(std::env::var("DICE_DATA_PATH")?);
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);

        let file_path = data_dir.join("dice-tray-data");
        let die_file = read_to_string(&file_path)?;

        let decoded_list: DiceDataList = serde_json::from_str(&die_file)?;
        Ok(decoded_list)
    }

    pub fn save_dice(&mut self, dice_bag: &DiceDataList, dir_path: Option<String>, filename : Option<String>) -> Result<(), Error>{
        dotenv::from_filename("rust_dice/src/.env").unwrap();

        let rel = PathBuf::from(std::env::var("DICE_DATA_PATH").unwrap());
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
        fs::create_dir_all(&data_dir)?;

        let file_path = data_dir.join(dice_bag.file_name.clone());
        let die_list_json = serde_json::to_string_pretty(&dice_bag)?;

        let mut save_file = fs::File::create(&file_path)?;
        save_file.write_all(die_list_json.as_bytes())?;

        Ok(())
    }
}
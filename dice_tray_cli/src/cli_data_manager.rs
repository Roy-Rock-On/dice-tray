use rust_dice::die::DiceDataList;
use anyhow::Error;

pub fn load_dice() -> Result<DiceDataList, Error>{
    use dotenv;
    use std::path::PathBuf;
    use std::fs::read_to_string;

    dotenv::from_filename("dice_tray_cli/src/.env")?;

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH")?);

    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);

    let file_path = data_dir.join("DiceData");
    let die_file = read_to_string(&file_path)?;

    let decoded_list: DiceDataList = serde_json::from_str(&die_file)?;
    Ok(decoded_list)
}

pub fn save_dice(dice_bag: &DiceDataList) -> Result<(), Error>{
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use dotenv;
    use serde_json;

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
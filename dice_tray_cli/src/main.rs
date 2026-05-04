mod cli_parser;
mod logger;

use logger::{detailed_log_tray, detailed_log_dice};
use cli_parser::{CliCommand};

use std::io::Write;
use anyhow::Error;

use rust_dice::die::DiceDataList; 
use rust_dice::die_allocator::{Allocator};

fn main() {
    println!("Welcome to Dice Tray!\n");
    let dice_list = match load_dice(){
        Ok(list) => list,
        Err(e) => {
            println!("Error loading dice from list. Error = {}", e);
            panic!();
        }
    };

    let mut die_allocator = Allocator::new();
    die_allocator.create_dice_from_list(dice_list).unwrap();

    println!("Dice bag sucessfuly loaded from file.");

    println!("---DICE BAG---");
    let _ =detailed_log_dice(die_allocator.get_dice());

	loop {
        println!();
		println!("Enter your dice-tray command now.");
        println!("Use 'help' to see list of commands and 'exit' to save your dice bag and quit.");
        print!("> ");
        let command = match get_command() {
            Ok(command) => command,
            Err(e) => {
                CliCommand::Error(
                    format!("Input failed with the following error: {}. Command not valid, please try again.", e)
                )
            }
        };

        println!();
        match command {
            CliCommand::Bag => {
                println!("---DICE BAG---");
                let _ = detailed_log_dice(die_allocator.get_dice());
            },
            CliCommand::Tray(id) =>{
                println!("Not yet implemented.");
            }
            CliCommand::Help => println!("Print the help here later. Once we figure out how this works."),
            CliCommand::Create => {
                match prompt_new_die() {
                    Ok((label, face_count, face_varience)) => {
                        match die_allocator.create_die(face_count, None, Some(label.clone()), face_varience) {
                            Ok(()) => println!(
                                "Created die '{}' with {} faces and variance {}.",
                                label,
                                face_count,
                                face_varience
                            ),
                            Err(e) => println!("Error creating die: {}", e),
                        }
                    }
                    Err(e) => println!("Error creating die: {}", e),
                }
            }
            CliCommand::Exit => {
                let dice_bag_data = die_allocator.build_die_data_list(None);
                match save_dice(&dice_bag_data){
                    Ok(())=> println!("Dice have been sucessfuly saved to {}. Goodbye!", dice_bag_data.file_name),
                    Err(e) => println!("Failed to save dice with error {}. Goodbye!", e) 
                }
                break;
            }
            CliCommand::Destroy(targets) => {
                match die_allocator.destroy_dice(&targets){
                    Ok(()) => println!("Sucessfuly removed dice at targets {}", targets),
                    Err(e) => println!("Error removing dice {}", e)
                }
            },
            CliCommand::Add(_, _) => println!("Not yet implemented."),
            CliCommand::Move(_, _) => println!("Not yet implemented."),
            CliCommand::Remove(_) => println!("Not yet implemented."),
            CliCommand::Error(e) => println!("{}", e)
        }
	}
}

fn load_dice() -> Result<DiceDataList, Error>{
    use dotenv;
    use std::path::PathBuf;
    use std::fs::read_to_string;

    dotenv::from_filename("dice_tray_cli/src/.env")?;

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH")?);

    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);

    let file_path = data_dir.join("die_test");
    let die_file = read_to_string(&file_path)?;

    let decoded_list: DiceDataList = serde_json::from_str(&die_file)?;
    Ok(decoded_list)
}

fn save_dice(dice_bag: &DiceDataList) -> Result<(), Error>{
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

fn get_command() -> Result<CliCommand, Error>{
    use std::io::{stdout, stdin};

    let mut input = String::new();
    
    stdout().flush().expect("failed to flush stdout");

    input.clear();
    stdin().read_line(&mut input)?;

    let command = input.trim();
    Ok(CliCommand::new(command))
} 

fn prompt_new_die() -> Result<(String, u32, u32), Error> {
    let label = prompt_nonempty("Enter die label")?;
    let face_count = prompt_u32("Enter face count")?;
    let face_varience = prompt_u32("Enter face variance")?;

    Ok((label, face_count, face_varience))
}

fn prompt_nonempty(prompt: &str) -> Result<String, Error> {
    loop {
        let input = prompt_line(prompt)?;
        if !input.is_empty() {
            return Ok(input);
        }

        println!("Input cannot be empty.");
    }
}

fn prompt_u32(prompt: &str) -> Result<u32, Error> {
    loop {
        let input = prompt_line(prompt)?;
        match input.parse::<u32>() {
            Ok(value) => return Ok(value),
            Err(_) => println!("Please enter a non-negative integer."),
        }
    }
}

fn prompt_line(prompt: &str) -> Result<String, Error> {
    use std::io::{stdin, stdout};

    let mut input = String::new();
    print!("{}: ", prompt);
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}


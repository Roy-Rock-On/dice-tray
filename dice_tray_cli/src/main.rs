mod cli_parser;
mod logger;

use logger::{detailed_log_dice, detailed_log_tray};
use cli_parser::{CliCommand};

use std::io::Write;
use std::cmp::Ordering;
use std::process::ExitCode;

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
    die_allocator.new_dice_from_list(dice_list).unwrap();

    println!("Dice bag sucessfuly loaded from file.");
    detailed_log_dice(die_allocator.get_dice_summary(Some(Ordering::Less))).unwrap();

	loop {
        println!();
		println!("Enter your dice-tray command now.");
        println!("Use 'help' to see list of commands and 'exit' to quit.");
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
            CliCommand::Help => println!("Print the help here later. Once we figure out how this works."),
            CliCommand::Exit => {
                println!("Saving dice and exiting dice-tray now. Goodbye!");
                break;
            }
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

fn get_command() -> Result<CliCommand, Error>{
    use std::io::{stdout, stdin};

    let mut input = String::new();
    
    stdout().flush().expect("failed to flush stdout");

    input.clear();
    stdin().read_line(&mut input)?;

    let command = input.trim();
    Ok(CliCommand::new(command))
} 


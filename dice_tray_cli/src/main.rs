mod cli_parser;
mod logger;
mod cli_command_handlers;
mod cli_data_manager;
mod dice_tay_command_parser;

use logger::{detailed_log_tray, detailed_log_dice};
use cli_parser::{CliCommand};

use std::io::Write;
use anyhow::Error;

use rust_dice::die::DiceDataList; 
use rust_dice::die_allocator::Allocator;
use cli_data_manager::{save_dice, load_dice};

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

    println!("Dice bag successfully loaded from file.");

    println!("---DICE BAG---");
    let _ = detailed_log_dice(die_allocator.get_dice());

	loop {
        println!();
		println!("Enter your dice-tray command now.");
        println!("Use 'help' to see list of commands and 'exit' to save your dice bag and quit.");
        print!("> ");
        //let command = get_command();
        //println!("Echoing Command {}", &command);


        println!();
	}
}



/* Exit LOGIC
    let dice_bag_data = die_allocator.build_die_data_list(None);
    match save_dice(&dice_bag_data){
        Ok(())=> println!("Dice have been successfully saved to {}. Goodbye!", dice_bag_data.file_name),
        Err(e) => println!("Failed to save dice with error {}. Goodbye!", e) 
    }
    break;
*/


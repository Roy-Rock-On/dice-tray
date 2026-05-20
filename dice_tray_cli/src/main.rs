mod logger;
mod dice_tray_cli_command_handlers;
mod cli_data_manager;
mod dice_tay_command_parser;

use logger::detailed_log_dice;
use rust_dice::die_allocator::Allocator;
use cli_data_manager::{save_dice, load_dice};
use dice_tray_cli_command_handlers::handle_command;
use dice_tay_command_parser::{get_command_string, process_commands};

fn main() {
    println!("Welcome to Dice Tray!\n");
    let dice_list = match load_dice(){
        Ok(list) => list,
        Err(e) => {
            println!("Error loading dice from list. Error = {}", e);
            panic!();
        }
    };

    let mut die_allocator = match Allocator::new(){
        Ok(alloc) => alloc,
        Err(e) => {
            println!("Dice allocator failed to initialize with error: {}", e);
            panic!()
        }
    };
    die_allocator.create_dice_from_list(dice_list).unwrap();

    println!("Dice bag successfully loaded from file.");

    println!("---DICE BAG---");
    let _ = detailed_log_dice(die_allocator.get_dice());

	'outer: loop {
        println!();
		println!("Enter your dice-tray command now.");
        println!("Use 'help' to see list of commands and 'exit' to save your dice bag and quit.");
        print!("{}> ", die_allocator.get_target_tray());
            let command_string = get_command_string();
            let commands = match process_commands(&command_string){
                Ok(commands) => commands,
                Err(e) => {
                    println!("No commands detected in input: {} | Errors: {:?}", command_string, e);
                    Vec::new()
                }
            };

        for command in commands {
            if !handle_command(&mut die_allocator, command){
                break 'outer;
            }
        }
    }
    println!("Goodbye!");
}



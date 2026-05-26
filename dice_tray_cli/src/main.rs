mod logger;
mod dice_tray_cli_command_handlers;
mod cli_data_manager;
mod dice_tay_command_parser;

use logger::detailed_log_dice;
use rust_dice::die_allocator::Allocator;
use cli_data_manager::DiceTrayDataManager;
use dice_tray_cli_command_handlers::handle_command;
use dice_tay_command_parser::{get_command_string, process_commands};

fn main(){
    println!("Welcome to Dice Tray!\n");

    //Create a data manager.
    let mut cli_data_manager = match DiceTrayDataManager::new(){
        Ok(data) => data,
        Err(e) => {
            println!("Failed to initializing data manager. PANIC! with error: {}", e);
            panic!()
        }
    };

    //Create a dice allocator.
    let mut die_allocator = match Allocator::new(){
        Ok(alloc) => alloc,
        Err(e) => {
            println!("Dice allocator failed to initialize with error: {}", e);
            panic!()
        }
    };

    match cli_data_manager.load_dice(){
        Ok(list) => die_allocator.create_dice_from_list(list).unwrap(),
        Err(e) => {
            println!("Error loading dice from list. Error = {}", e);
            println!("Generating default dice data now.");
            die_allocator.generate_default_dice();
        }
    };

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
    println!("Saving dice data now.");
    match cli_data_manager.save_dice(&die_allocator.build_dice_data()){
        Ok(()) => println!("Saving dice data now. Directory: {}, Filename: {}", cli_data_manager.get_directory_str(), cli_data_manager.get_filename_str()),
        Err(e) => println!("Failed to save dice data. Error {}", e)
    }
    println!("Goodbye!");
}



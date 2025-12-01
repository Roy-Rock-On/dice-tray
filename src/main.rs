use std::io;
use dice_tray::tray::{Tray};
use dice_tray::logger::log_tray;
use dice_tray::cli_parser::{parse_dice_tray_commands, parse_add_command, parse_roll_command, parse_drop_command};
use dice_tray::cli_parser::{DiceTrayCommandType, Targets};
use dice_tray::dice::DieResultType;


fn main() {
    let mut active_tray = Tray::new();
    println!("Welcome to Dice Tray!");

    log_tray(&active_tray);
    loop {
        println!("Enter commands (type \"help\" for options):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let dice_tray_commands = parse_dice_tray_commands(&input);
        println!("count of commands parsed: {}", dice_tray_commands.len());
        if dice_tray_commands.is_empty() {
            println!("No valid commands found. Please try again.");
            continue;
        }

        for command in dice_tray_commands {
            match command.command_type {
                DiceTrayCommandType::Add => {
                    if let Some(command_string) = command.command_string {
                        let new_dice = parse_add_command(Some(&command_string));
                        match new_dice {
                            Some(dice) => {
                                println!("Adding {} dice to the tray.", dice.len());
                                active_tray.add_dice(dice);
                            }
                            None => println!("No dice parsed from command: {}", command_string),
                        }
                    }
                },
                DiceTrayCommandType::Roll => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                println!("Rolling dice with identity: {}", id);
                                match active_tray.set_result_type_by_id(&id, DieResultType::Face) {
                                    Ok(()) => {
                                        match active_tray.roll_by_id(&id) {
                                            Ok(()) => println!("Rolled all dice with identity: {}", id),
                                            Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                println!("Rolling die at index: {}", index);
                                match active_tray.set_result_type_at(*index, DieResultType::Face) {
                                    Ok(()) => {
                                        match active_tray.roll_at(*index) {
                                            Ok(()) => println!("Rolled die at index: {}", index),
                                            Err(e) => println!("Error rolling die at index {}: {}", index, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for die at index {}: {}", index, e),
                                }
                            });
                        }

                        if targets.is_empty(){
                            println!("Rolling all dice in the tray.");
                            active_tray.set_all_result_type(DieResultType::Face);
                            active_tray.roll_all();
                        }
                    }  
                },
                DiceTrayCommandType::ReRollBest => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                println!("Re-rolling dice with identity for best result: {}", id);
                                match active_tray.set_result_type_by_id(&id, DieResultType::Best) {
                                    Ok(()) => {
                                        match active_tray.roll_by_id(&id) {
                                            Ok(()) => println!("Re-rolled all dice with identity for best result: {}", id),
                                            Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                println!("Re-rolling die at index for best result: {}", index);
                                match active_tray.set_result_type_at(*index, DieResultType::Best) {
                                    Ok(()) => {
                                        match active_tray.roll_at(*index) {
                                            Ok(()) => println!("Re-rolled die at index for best result: {}", index),
                                            Err(e) => println!("Error rolling die at index {}: {}", index, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for die at index {}: {}", index, e),
                                }
                            });
                        }

                        if targets.is_empty(){
                            println!("Re-rolling all dice in the tray for best results.");
                            active_tray.set_all_result_type(DieResultType::Best);
                            active_tray.roll_all();
                        }
                    }
                },
                DiceTrayCommandType::ReRollWorst => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                println!("Re-rolling dice with identity for worst result: {}", id);
                                match active_tray.set_result_type_by_id(&id, DieResultType::Worst) {
                                    Ok(()) => {
                                        match active_tray.roll_by_id(&id) {
                                            Ok(()) => println!("Re-rolled all dice with identity for worst result: {}", id),
                                            Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                println!("Re-rolling die at index for worst result: {}", index);
                                match active_tray.set_result_type_at(*index, DieResultType::Worst) {
                                    Ok(()) => {
                                        match active_tray.roll_at(*index) {
                                            Ok(()) => println!("Re-rolled die at index for worst result: {}", index),
                                            Err(e) => println!("Error rolling die at index {}: {}", index, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for die at index {}: {}", index, e),
                                }
                            });
                        }

                        if targets.is_empty(){
                            println!("Re-rolling all dice in the tray for worst results.");
                            active_tray.set_all_result_type(DieResultType::Worst);
                            active_tray.roll_all();
                        }
                    }
                },
                DiceTrayCommandType::Explode => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                println!("Exploding dice with identity: {}", id);
                                match active_tray.set_result_type_by_id(&id, DieResultType::Sum) {
                                    Ok(()) => {
                                        match active_tray.roll_by_id(&id) {
                                            Ok(()) => println!("Exploded all dice with identity: {}", id),
                                            Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                println!("Exploding die at index: {}", index);
                                match active_tray.set_result_type_at(*index, DieResultType::Sum) {
                                    Ok(()) => {
                                        match active_tray.roll_at(*index) {
                                            Ok(()) => println!("Exploded die at index: {}", index),
                                            Err(e) => println!("Error rolling die at index {}: {}", index, e),
                                        }
                                    },
                                    Err(e) => println!("Error setting result type for die at index {}: {}", index, e),
                                }
                            });
                        }

                        if targets.is_empty(){
                            println!("Exploding all dice in the tray.");
                            active_tray.set_all_result_type(DieResultType::Sum);
                            active_tray.roll_all();
                        }
                    }
                },
                DiceTrayCommandType::Drop => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_drop_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                println!("Dropping all dice with identity: {}", id);
                                active_tray.remove_by_id(&id);
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().rev().for_each(|index| {
                                match active_tray.remove_at(*index) {
                                    Some(die) => println!("Dropped die at index {}: {}", index, die.get_id()),
                                    None => println!("Error dropping die at index {}: Index out of bounds", index),
                                }
                            });
                        }

                        if targets.is_empty(){
                            active_tray.clear();
                            println!("Cleared all dice from the tray.");
                        }
                    }
                },
                DiceTrayCommandType::Help => {
                    println!("Available commands:");
                    println!("-a(Add): Add dice to the tray using standard dice notation (e.g., '2d6' for two six-sided dice).");
                    println!("-r(Roll): Roll dice in the tray by targeting them using $DieID or @<dieIndex>. If no target is provided, all dice will be rolled.");
                    println!("-rb(Re-roll Best): Re-rolls the targeted dice and keeps the best result.");
                    println!("-rw(Re-roll Worst): Re-rolls the targeted dice and keeps the worst result.");
                    println!("-d(Drop): Remove dice from the tray by targeting them using $DieID or @<dieIndex>. If no target is provided, all dice will be removed.");
                    println!("-h(Help): Show this help message.");
                    println!("-e(Exit): Exit the application.");
                    println!("You can combine multiple commands in one line, separated by spaces.");
                    println!("$<id> is used to refrence the id of a die.");
                    println!("@<index1,index2> is used to refrence the index of dice in the tray.");
                },
                DiceTrayCommandType::Exit => {
                    println!("Exiting Dice Tray. Goodbye!");
                    return;
                }
            }
        }
        log_tray(&active_tray);
    }
}

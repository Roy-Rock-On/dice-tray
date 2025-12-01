use std::io;
use dice_tray::tray::{Tray};
use dice_tray::logger::log_tray;
use dice_tray::cli_parser::{parse_dice_tray_commands, parse_add_command, parse_roll_command, parse_drop_command};
use dice_tray::cli_parser::DiceTrayCommandType;


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
                        let (identity_flags, index_flags) = parse_roll_command(Some(&command_string));
                        match (identity_flags, index_flags) {
                            (Some(ids), None) => {
                                for id in ids {
                                    println!("Rolling dice with identity: {}", id);
                                    match active_tray.roll_by_id(&id) {
                                        Ok(()) => println!("Rolled all dice with identity: {}", id),
                                        Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                    }
                                }
                            },
                            (None, Some(indices)) => {
                                for index in indices {
                                    match active_tray.roll_at(index) {
                                        Ok(()) => println!("Rolled die at index: {}", index),
                                        Err(e) => println!("Error rolling die at index {}: {}", index, e),
                                    }
                                }
                            },
                            (Some(ids), Some(indices)) => {
                                for id in ids {
                                    match active_tray.roll_at_identities(&id, &indices) {
                                        Ok(()) => println!("Rolled dice with identity: {} at specified indices {:?}", id, indices),
                                        Err(e) => println!("Error rolling dice with identity {} at specified indices: {}", id, e),
                                    }
                                }
                            },
                            (None, None) => {
                                println!("Rolling all dice in the tray.");
                                active_tray.roll_all();
                            }
                        }
                    } else {
                        println!("Rolling all dice in the tray.");
                        active_tray.roll_all();
                    }   
                },

                DiceTrayCommandType::Drop => {
                    if let Some(command_string) = command.command_string {
                        let (identity_flags, drop_indices) = parse_drop_command(Some(&command_string));
                        match (identity_flags, drop_indices) {
                            (Some(ids), None) => {
                                for id in ids {
                                    println!("Dropping all dice with identity: {}", id);
                                    active_tray.remove_by_id(&id);
                                }
                            },
                            (None, Some(indices)) => {
                                for index in indices.iter().rev() {
                                    match active_tray.remove_at(*index) {
                                        Some(die) => println!("Dropped die at index {}: {}", index, die.get_id()),
                                        None => println!("Error dropping die at index {}: Index out of bounds", index),
                                    }
                                }
                            },
                            (Some(ids), Some(indices)) => {
                                for id in ids {
                                    println!("Dropping dice with identity: {} at specified indices {:?}", id, indices);
                                    for index in indices.iter().rev() {
                                        match active_tray.get_dice().get(*index) {
                                            Some(die) if die.get_id() == id => {
                                                match active_tray.remove_at(*index) {
                                                    Some(removed_die) => println!("Dropped die at index {}: {}", index, removed_die.get_id()),
                                                    None => println!("Error dropping die at index {}: Index out of bounds", index),
                                                }
                                            },
                                            _ => println!("No die with identity {} at index {}", id, index),
                                        }
                                    }
                                }
                            },
                            (None, None) => {
                                active_tray.clear();
                                println!("Cleared all dice from the tray.");
                            }
                        }
                    }
                }
                DiceTrayCommandType::Mod => {
                   todo!("Modify dice functionality not yet implemented");
                },
                DiceTrayCommandType::Help => {
                    println!("Available commands:");
                    println!("-r(Roll): Add dice to the tray using standard dice notation (e.g., '2d6' for two six-sided dice).");
                    println!("-c(Clear): Clear all dice from the tray.");
                    println!("-m(Mod): Modify dice in the tray.");
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

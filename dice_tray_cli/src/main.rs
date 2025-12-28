mod cli_parser;
mod logger;

use cli_parser::{
    DiceTrayCommandType, 
    Targets, 
    parse_add_command,
    parse_dice_tray_commands,
    parse_drop_command,
    parse_roll_command,
};

use rust_dice::settings::{DICE_TRAY_SETTINGS};
use logger::log_tray;
use std::io;
use rust_dice::dice::DieResultType;
use rust_dice::tray::Tray;

use crate::cli_parser::parse_custom_command;

fn main() {
    let active_tray = Tray::new();
    println!("Welcome to Dice Tray!");

    let settings = DICE_TRAY_SETTINGS.lock().unwrap();
    settings.load();

    log_tray(&active_tray);
    dice_loop(active_tray);

    match settings.save() {
        Ok(_) => println!("Tray Settings saved. Goodbye."),
        Err(error_string) => println!("Failed to save tray settings. Error: {}", error_string),
    }
}

fn dice_loop(mut active_tray: Tray) {
    loop {
        println!("Enter commands (type \"help\" for options):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let dice_tray_commands = parse_dice_tray_commands(&input);
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
                }
                DiceTrayCommandType::Roll => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                match active_tray.roll_by_id(&id, DieResultType::Face) {
                                    Ok(()) => println!("Rolled all dice with identity {}", id),
                                    Err(e) => {
                                        println!("Error rolling dice with identity {}: {}", id, e)
                                    }
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                match active_tray.roll_at(*index, DieResultType::Face) {
                                    Ok(()) => println!("Rolled die at index {}", index),
                                    Err(e) => {
                                        println!("Error rolling die at index {}: {}", index, e)
                                    }
                                }
                            });
                        }

                        if targets.is_empty() {
                            println!("Rolling all dice in the tray.");
                            active_tray.roll_all(DieResultType::Face);
                        }
                    }
                }
                DiceTrayCommandType::ReRollBest => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                match active_tray.roll_by_id(&id, DieResultType::Best) {
                                    Ok(()) => println!("Re-rolled all dice with identity {}, keeping the best result.", id),
                                    Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                match active_tray.roll_at(*index, DieResultType::Best) {
                                    Ok(()) => println!(
                                        "Re-rolled die at index {}, keeping the best result.",
                                        index
                                    ),
                                    Err(e) => {
                                        println!("Error rolling die at index {}: {}", index, e)
                                    }
                                }
                            });
                        }

                        if targets.is_empty() {
                            println!("Re-rolling all dice in the tray, keeping the best results.");
                            active_tray.roll_all(DieResultType::Best);
                        }
                    }
                }
                DiceTrayCommandType::ReRollWorst => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                match active_tray.roll_by_id(&id, DieResultType::Worst) {
                                    Ok(()) => println!("Re-rolled all dice with identity {}, keeping the worst result.", id),
                                    Err(e) => println!("Error rolling dice with identity {}: {}", id, e),
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                match active_tray.roll_at(*index, DieResultType::Worst) {
                                    Ok(()) => println!(
                                        "Re-rolled die at index {}, keeping the worst result.",
                                        index
                                    ),
                                    Err(e) => {
                                        println!("Error rolling die at index {}: {}", index, e)
                                    }
                                }
                            });
                        }

                        if targets.is_empty() {
                            println!("Re-rolling all dice in the tray, keeping the worst results.");
                            active_tray.roll_all(DieResultType::Worst);
                        }
                    }
                }
                DiceTrayCommandType::Explode => {
                    if let Some(command_string) = command.command_string {
                        let targets: Targets = parse_roll_command(Some(&command_string));
                        if let Some(identity_flags) = targets.get_identity_flags() {
                            identity_flags.iter().for_each(|id| {
                                match active_tray.roll_by_id(&id, DieResultType::Sum) {
                                    Ok(()) => println!("Exploded all dice with identity {}.", id),
                                    Err(e) => {
                                        println!("Error rolling dice with identity {}: {}", id, e)
                                    }
                                }
                            });
                        }

                        if let Some(index_flags) = targets.get_index_flags() {
                            index_flags.iter().for_each(|index| {
                                match active_tray.roll_at(*index, DieResultType::Sum) {
                                    Ok(()) => println!("Exploded die at index {}.", index),
                                    Err(e) => {
                                        println!("Error rolling die at index {}: {}", index, e)
                                    }
                                }
                            });
                        }

                        if targets.is_empty() {
                            println!("Exploding all dice in the tray.");
                            active_tray.roll_all(DieResultType::Sum);
                        }
                    }
                }
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
                                    Some(die) => {
                                        println!("Dropped die at index {}: {}", index, die.get_id())
                                    }
                                    None => println!(
                                        "Error dropping die at index {}: Index out of bounds",
                                        index
                                    ),
                                }
                            });
                        }

                        if targets.is_empty() {
                            active_tray.clear();
                            println!("Cleared all dice from the tray.");
                        }
                    }
                }
                DiceTrayCommandType::Custom => {
                    if let Some(command_string) = &command.command_string {
                        match parse_custom_command(command_string) {
                            Ok(result_table) => {
                                let mut settings = DICE_TRAY_SETTINGS.lock().unwrap();
                                settings.add_result_table(result_table);
                            }
                            Err(error) => {
                                println!("Error creating custom dice table: {}", error)
                            }
                        }
                    }
                }
                DiceTrayCommandType::Help => {
                    println!("Available commands:");
                    println!(
                        "-a(Add): Add dice to the tray using standard dice notation (e.g., '2d6' for two six-sided dice)."
                    );
                    println!(
                        "-r(Roll): Roll dice in the tray by targeting them using $DieID or @<dieIndex>. If no target is provided, all dice will be rolled."
                    );
                    println!(
                        "-rb(Re-roll Best): Re-rolls the targeted dice and keeps the best result."
                    );
                    println!(
                        "-rw(Re-roll Worst): Re-rolls the targeted dice and keeps the worst result."
                    );
                    println!(
                        "-e(Explode): Re-rolls the targeted dice and adds the new result to the previous result."
                    );
                    println!(
                        "-d(Drop): Remove dice from the tray by targeting them using $DieID or @<dieIndex>. If no target is provided, all dice will be removed."
                    );
                    //println!("-c(Custom): Creates a new dice lookup table that can be used for custom dice types. Requires an identity flag (e.g \"$DICENAME\") and a set of strings seperated with whitespace.");
                    println!("-h(Help): Show this help message.");
                    println!("-e(Exit): Exit the application.");
                    println!("You can combine multiple commands in one line, separated by spaces.");
                    println!("$<id> is used to refrence the id of a die.");
                    println!("@<index1,index2> is used to refrence the index of dice in the tray.");
                }
                DiceTrayCommandType::Exit => {
                    println!("Exiting Dice Tray. Goodbye!");
                    return;
                }
            }
        }
        log_tray(&active_tray);
    }
}

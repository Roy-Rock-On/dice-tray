use rust_dice::{die::Die, die_allocator::{Allocator, DiceSummary, DieSummary}, die_tray::TraySummary};
use anyhow::Error;

use crate::cli_parser::CliCommand;

use std::cmp::Ordering;

pub enum CommandResult{
    AllDice(DiceSummary),
    ShowTray(TraySummary),
    AllTrays(Vec<TraySummary>),
    NewDie(DieSummary),
    Error(String),
    HelpRequest,
    QuitRequest,
}

pub fn handle_command(allocator: &mut Allocator, command: CliCommand) -> CommandResult{
    match command {
        CliCommand::Bag => {
            CommandResult::AllDice(allocator.get_dice_summaries())
        },
        CliCommand::Tray(id) =>{
            match id {
                Some(tray_id) => {
                    match allocator.sort_tray(tray_id, Ordering::Less) {
                        Ok(summary) => CommandResult::ShowTray(summary),
                        Err(e) => CommandResult::Error(format!("Error retrieving tray {}: {}", tray_id, e))
                    }
                }
                None => {
                    let tray_ids = allocator.get_tray_ids();
                    let mut tray_summaries = Vec::new();
                    for tray_id in tray_ids {
                        if let Ok(summary) = allocator.sort_tray(tray_id, Ordering::Less) {
                            tray_summaries.push(summary);
                        }
                    }

                    if tray_summaries.is_empty(){
                        CommandResult::Error(format!("No trays found in allocator."))
                    }
                    else{
                        CommandResult::AllTrays(tray_summaries)
                    }
                }
            }
        },
        CliCommand::New => {
            if let Ok(label) = prompt_nonempty("Enter a label for your new tray: ") {
                let summary = allocator.create_tray(Some(label.clone()));
                CommandResult::ShowTray(summary)
            }
            else{
                CommandResult::Error(format!("Error creating tray. Every tray requires a unique label."))
            }
        },
        CliCommand::Help => CommandResult::HelpRequest,
        CliCommand::Create => {
            match prompt_new_die() {
                Ok((label, face_count, face_varience)) => {
                    match allocator.create_die(face_count, None, Some(label.clone()), face_varience) {
                        Ok(die_summary) => CommandResult::NewDie(die_summary),
                        Err(e) => CommandResult::Error(format!("Failed to create new die with error: {}", e))
                    }
                },
                Err(e) => {
                    CommandResult::Error(format!("Failed to create new die with error: {}", e))     
                }
            }
        },
        CliCommand::Exit => {
            CommandResult::QuitRequest
        },
        CliCommand::Destroy(targets) => {
            let die_ids= allocator.get_die_ids_from_targets(targets).unwrap_or(Vec::new());
            match allocator.destroy_dice(die_ids){
                Ok(summary) => CommandResult::AllDice(summary),
                Err(e) => CommandResult::Error(format!("Error removing dice: {}", e))
            }
        },
        CliCommand::Add(die_targets, tray_id) => {
            let die_ids = match allocator.get_die_ids_from_targets(die_targets){
                Some(ids) => ids,
                None => return CommandResult::Error(format!("Error adding dice. No dice found at provided targets to add."))
            };

            if die_ids.is_empty() {
                return CommandResult::Error(format!("Add dice failed. No dice matched targets."))
            }
            else{
                let mut added_count = 0usize;
                for die_id in die_ids {
                    if allocator.add_die_reader(die_id, tray_id).is_ok() {
                        added_count += 1;
                    }
                }   
                println!("Added {} die_reader(s) to tray {}.", added_count, tray_id);
                match allocator.get_tray_summary(tray_id){
                    Ok(summary) => CommandResult::ShowTray(summary),
                    Err(e) => CommandResult::Error(format!("Add dice failed with error {}", e))
                }
            }
        }
        CliCommand::Move(die_reader_targets, from_tray_id, to_tray_id) => {
            todo!("Do this someday");
        },
        CliCommand::Error(e) => CommandResult::Error(format!("Error handling command: {}", e))
    }
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
    use std::io::Write;

    let mut input = String::new();
    print!("{}: ", prompt);
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
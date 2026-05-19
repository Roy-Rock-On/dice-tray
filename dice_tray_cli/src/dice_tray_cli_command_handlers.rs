use rust_dice::{die::Die, die_allocator::{Allocator, DiceSummary, DieSummary}, die_tray::TraySummary};
use anyhow::{Error, bail};

use crate::dice_tay_command_parser::{AddDiceArgs, CreateDieArgs, DiceTrayCommand, MoveDiceArgs, RollDiceArgs};
use crate::logger::{detailed_log_dice, detailed_log_tray};

pub enum CommandResult{
    AllDice(DiceSummary),
    ShowTray(TraySummary),
    AllTrays(Vec<TraySummary>),
    NewDie(DieSummary),
    Error(String),
    HelpRequest,
    QuitRequest,
}

pub fn handle_command(allocator: &mut Allocator, command: DiceTrayCommand) -> bool {
    let mut continue_program = true;
    let result = match command{
        DiceTrayCommand::AddDice(add_args) => process_add_dice(allocator, add_args),
        DiceTrayCommand::DeleteDice(delete_args) => todo!(),
        DiceTrayCommand::DeleteTray(tray_label) =>todo!(),
        DiceTrayCommand::Load(load_path) => todo!(),
        DiceTrayCommand::Save(save_path) => todo!(),
        DiceTrayCommand::NewDie(new_die_args) =>todo!(),
        DiceTrayCommand::MoveDice(move_dice_args) => todo!(),
        DiceTrayCommand::NewTray(new_tray_label) => process_new_tray(allocator, new_tray_label),
        DiceTrayCommand::RemoveDice(remove_dice_args) => todo!(),
        DiceTrayCommand::RollDice(roll_dice_args) => todo!(),
        DiceTrayCommand::ShowDiceBag => todo!(),
        DiceTrayCommand::ShowTray(tray_label) => todo!(),
        DiceTrayCommand::Help => process_help_command(),
        DiceTrayCommand::Exit => return false,
    };

    if let Err(e) = result{
        println!("Command failed. | Errors = {}", e);
    };

    continue_program
}

fn process_new_dice(allocator: &mut Allocator, new_die_args: CreateDieArgs){
    let faces = match new_die_args.faces{
        Some(num) => num,
        None => 6
    };

    let var = match new_die_args.variance{
        Some(num) => num,
        None => 25
    };

    match allocator.create_die(faces, None, new_die_args.label.clone(), var){
        Ok(summary) => {
            println!("Created a new die!");
            summary.print();
        }
        Err(e) => println!("Failed to create new die with arguments: {:?} | Error: {:?}", new_die_args, e)
    };
}

fn process_new_tray(allocator: &mut Allocator, tray_label: String) -> anyhow::Result<()>{
    if let Ok(new_tray_summary) = allocator.create_tray(tray_label.clone()){
        println!("New tray created with label: {}", &tray_label);
        detailed_log_tray(new_tray_summary);
        Ok(())
    }
    else{
        bail!("Failed to create a new tray. There is already a tray with label {}", &tray_label);
    }
}

fn process_add_dice(allocator: &mut Allocator, add_args: AddDiceArgs) -> anyhow::Result<()>{
    if let Some(dice_targets) = allocator.get_die_ids_from_targets(&add_args.dice_targets){
        let mut count = 0;
        for id in dice_targets{
            for _ in 0..add_args.number{
                let reader_id = allocator.add_die_reader(id,&add_args.tray_target)?;
                if add_args.should_roll{
                    allocator.roll_at(&add_args.tray_target, &[reader_id])?
                }
                count += 1;
            }
        }


        let tray_summary = allocator.get_tray_summary_at(&add_args.tray_target)?;
        println!("Added {} dice to tray: {}", count, tray_summary.get_label());
        detailed_log_tray(tray_summary);
        Ok(())
    } else {
        bail!("No dice targets found when processing add command. Targets provided: {:?}", &add_args.dice_targets)
    }
}

fn process_help_command() -> anyhow::Result<()>{
    println!("Here is where the help will go. Whenever I write it.");
    Ok(())
}
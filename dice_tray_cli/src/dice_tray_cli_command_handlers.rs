use std::any;

use rust_dice::die_allocator::Allocator;
use anyhow::bail;
use rust_dice::die_targets::DiceTargets;

use crate::dice_tay_command_parser::{AddDiceArgs, CreateDieArgs, DiceTrayCommand, MoveDiceArgs, RemoveDiceArgs, RollDiceArgs};
use crate::logger::{detailed_log_dice, detailed_log_tray, detailed_move_summary};

pub fn handle_command(allocator: &mut Allocator, command: DiceTrayCommand) -> bool {
    let mut continue_program = true;
    let result = match command{
        DiceTrayCommand::AddDice(add_args) => process_add_dice(allocator, add_args),
        DiceTrayCommand::DeleteDice(delete_args) => process_delete_dice(allocator, &delete_args),
        DiceTrayCommand::DeleteTray(tray_label) => process_delete_tray(allocator, &tray_label),
        DiceTrayCommand::Load(load_path) => todo!(),
        DiceTrayCommand::Save(save_path) => todo!(),
        DiceTrayCommand::NewDie(new_die_args) => process_new_die(allocator, new_die_args),
        DiceTrayCommand::MoveDice(move_dice_args) => process_move_dice(allocator, &move_dice_args),
        DiceTrayCommand::NewTray(new_tray_label) => process_new_tray(allocator, new_tray_label),
        DiceTrayCommand::RemoveDice(remove_dice_args) => process_remove_dice(allocator, remove_dice_args),
        DiceTrayCommand::RollDice(roll_dice_args) => process_roll_dice(allocator, roll_dice_args),
        DiceTrayCommand::ShowDiceBag => process_show_dice_bag(allocator),
        DiceTrayCommand::ShowTray(tray_label) => process_show_tray(allocator, &tray_label),
        DiceTrayCommand::TargetTray(tray_label) => process_target_tray(allocator, &tray_label),
        DiceTrayCommand::Help => process_help_command(),
        DiceTrayCommand::Exit => return false,
    };

    if let Err(e) = result{
        println!("Command failed. | Errors = {}", e);
    };

    continue_program
}

fn process_new_die(allocator: &mut Allocator, new_die_args: CreateDieArgs) -> anyhow::Result<()> {
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
        },
        Err(e) => bail!("Failed to create new die with arguments: {:?} | Error: {:?}", new_die_args, e)
    }
    Ok(())
}

fn process_new_tray(allocator: &mut Allocator, tray_label: String) -> anyhow::Result<()>{
    if let Ok(new_tray_summary) = allocator.create_tray(tray_label.clone()){
        println!("New tray created with label: {}", &tray_label);
        detailed_log_tray(new_tray_summary)?;
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
                let reader_id = allocator.add_die_reader(add_args.should_roll, id,&add_args.tray_target)?;
                if add_args.should_roll{
                    allocator.roll_at(add_args.tray_target.as_deref(), &[reader_id])?
                }
                count += 1;
            }
        }

        let tray_summary = allocator.get_tray_summary_at(&add_args.tray_target)?;
        println!("Added {} dice to tray: {}", count, tray_summary.get_label());
        detailed_log_tray(tray_summary)?;
        Ok(())
    } else {
        bail!("No dice targets found when processing add command. Targets provided: {:?}", &add_args.dice_targets)
    }
}

fn process_delete_dice(allocator: &mut Allocator, targets: &DiceTargets) -> anyhow::Result<()> {
    let target_index = match allocator.get_die_ids_from_targets(targets){
        Some(targets) => targets,
        None => bail!("Cannot delete dice. No targets found for {:?}", targets)
    };

    println!("Destroying dice at {:?}", target_index);
    allocator.destroy_dice(target_index)?;

    Ok(())
}

fn process_delete_tray(allocator: &mut Allocator, tray_label: &str) -> anyhow::Result<()> {
    allocator.destroy_tray(tray_label)?;
    println!("Deleted tray with label {}", tray_label);
    Ok(())
}

fn process_move_dice(allocator: &mut Allocator, move_args: &MoveDiceArgs) -> anyhow::Result<()> {
    let from_tray_label = match &move_args.from_tray{
        Some(label) => label,
        None => &allocator.get_target_tray().to_string()
    };

    println!("DEBUG: 'move dice args' = {:?}", move_args);

    let to_tray_label = move_args.to_tray.as_deref();

    let reader_targets = allocator.get_reader_ids_by_targets(&from_tray_label, &move_args.dice_targets)?; 
    println!("DEBUG: while processing move reader targets = {:?}", &reader_targets);
    let move_summary = allocator.move_reader(true, &from_tray_label, &reader_targets, to_tray_label)?;

    println!("Moved {} dice from tray: {} to tray: {:?}", reader_targets.len(), from_tray_label, to_tray_label);
    detailed_move_summary(move_summary)?;
    Ok(())
}

fn process_remove_dice(allocator: &mut Allocator, remove_args: RemoveDiceArgs) -> anyhow::Result<()> {
    let from_tray_label = match &remove_args.from_tray{
        Some(label) => label,
        None => &allocator.get_target_tray().to_string()
    };

    let reader_targets = allocator.get_reader_ids_by_targets(&from_tray_label, &remove_args.dice_targets)?; 
    let summery = allocator.move_reader(false, from_tray_label, &reader_targets, None)?;

    println!("Removing {} dice from tray: {}", reader_targets.len(), from_tray_label);
    detailed_move_summary(summery)?;
    Ok(())
}

fn process_roll_dice(allocator: &mut Allocator, roll_args: RollDiceArgs) -> anyhow::Result<()> {
    let in_tray_label = match &roll_args.tray_target{
        Some(label) => label,
        None => &allocator.get_target_tray().to_string()
    };
    
    let reader_targets = allocator.get_reader_ids_by_targets(&in_tray_label, &roll_args.dice_targets)?;
    
    allocator.roll_at(Some(in_tray_label), &reader_targets)?;
 
    println!("Rolled {} dice in tray {}", reader_targets.len(), in_tray_label);
    let summery = allocator.get_tray_summary(&in_tray_label, None)?;
    detailed_log_tray(summery)?;
    Ok(())
}

fn process_show_dice_bag(allocator: &mut Allocator) -> anyhow::Result<()>{
    println!("---DICE BAG---");
    detailed_log_dice(allocator.get_dice())?;
    Ok(())
}

fn process_show_tray(allocator: &mut Allocator, tray_label: &str) -> anyhow::Result<()> {
    let summary = allocator.get_tray_summary(tray_label, None)?;
    println!("Showing tray:");
    detailed_log_tray(summary)?;
    Ok(())
}

fn process_target_tray(allocator: &mut Allocator, tray_label: &str) -> anyhow::Result<()> {
    let summary = allocator.get_tray_summary(tray_label, None)?;
    println!("Showing tray:");
    detailed_log_tray(summary)?;
    allocator.set_target_tray(tray_label.to_string())?;
    Ok(())
}

fn process_help_command() -> anyhow::Result<()>{
    println!("Here is where the help will go. Whenever I write it.");
    Ok(())
}



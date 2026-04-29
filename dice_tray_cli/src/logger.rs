use anyhow::Error;
use cli_table::{Table, WithTitle, format::Justify, print_stdout};
use rust_dice::{die::DieSummary, die_allocator::TraySummary};

#[derive(Table)]
struct DetailedDiceState {
    #[table(title = "Index", justify = "Justify::Center")]
    index: usize,
    #[table(title = "Label", justify = "Justify::Center")]
    label: String,
    #[table(title = "Face Count", justify = "Justify::Center")]
    faces_string: String,
    #[table(title = "Current Face", justify = "Justify::Center")]
    current_face_string: String,
    #[table(title = "Result", justify = "Justify::Center")]
    result_string: String,
}

pub fn detailed_log_dice(dice_summary: Vec<DieSummary>) -> Result<(), Error>{
    let dice_states: Vec<DetailedDiceState> = dice_summary
        .iter()
        .map(|die| DetailedDiceState{
            index: die.die_id,
            label: die.die_label.to_string(),
            faces_string: die.total_faces.to_string(),
            current_face_string: die.current_face.to_string(),
            result_string: die.result.to_string(),
        })
        .collect();

    print_stdout(dice_states.with_title())?;
    Ok(())
}

/// Logs the current state of the tray to the console. In table format. Using cli-table crate.
pub fn detailed_log_tray(summery : TraySummary) -> Result<(), Error>{
    let dice_states: Vec<DetailedDiceState> = summery.tray_dice
        .iter()
        .map(|(die)| DetailedDiceState {
            index: die.die_id,
            label: die.die_label.to_string(),
            faces_string: die.total_faces.to_string(),
            current_face_string: die.current_face.to_string(),
            result_string: die.result.to_string(),
        })
        .collect();

    println!("---ID: {} TRAY: {}---", summery.tray_id, summery.tray_label);
    print_stdout(dice_states.with_title())?;

    Ok(())
}


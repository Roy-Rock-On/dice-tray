use anyhow::Error;
use cli_table::{Table, WithTitle, format::Justify, print_stdout};
use rust_dice::die_tray::{MoveSummary, TraySummary};
use rust_dice::die::Die;
use rust_dice::die_reader::DieReader;
use std::cmp::Ordering;

#[derive(Table)]
struct DetailedDiceState {
    #[table(title = "ID", justify = "Justify::Center")]
    id: String,
    #[table(title = "Label", justify = "Justify::Center")]
    label: String,
    #[table(title = "Face Count", justify = "Justify::Center")]
    face_count: u32,
    #[table(title = "Current Face", justify = "Justify::Center")]
    current_face_string: u32,
    #[table(title = "Result", justify = "Justify::Center")]
    result_string: String,
}

impl PartialEq for DetailedDiceState {
    fn eq(&self, other: &Self) -> bool {
        self.face_count == other.face_count
            && self.current_face_string == other.current_face_string
            && self.result_string == other.result_string
            && self.label == other.label
            && self.id == other.id
    }
}

impl Eq for DetailedDiceState {}

impl PartialOrd for DetailedDiceState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DetailedDiceState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.face_count
            .cmp(&other.face_count)
            .then_with(|| self.current_face_string.cmp(&other.current_face_string))
            .then_with(|| self.result_string.cmp(&other.result_string))
            .then_with(|| self.label.cmp(&other.label))
            .then_with(|| self.id.cmp(&other.id))
    }
}

pub fn detailed_log_dice(dice_summary: Vec<&Die>) -> Result<(), Error>{
    let mut dice_states: Vec<DetailedDiceState> = dice_summary
        .iter()
        .map(|die: &&Die| DetailedDiceState{
            id: die.get_id().to_string(),
            label: die.get_label().to_string(),
            face_count: die.get_face_count(),
            current_face_string: die.get_current_face(),
            result_string: die.get_current_result().to_string(),
        })
        .collect();

    dice_states.sort();

    print_stdout(dice_states.with_title())?;
    Ok(())
}

/// Logs the current state of the tray to the console. In table format. Using cli-table crate.
pub fn detailed_log_tray(summery : TraySummary) -> Result<(), Error>{
    let dice_states: Vec<DetailedDiceState> = summery.get_dice()
        .iter()
        .map(|die: &DieReader| DetailedDiceState {
            id: die.get_reader_id().to_string(),
            label: die.get_label().to_string(),
            face_count: die.get_face_count(),
            current_face_string: die.get_current_face(),
            result_string: die.get_current_result().to_string(),
        })
        .collect();

    println!("---TRAY: {}---", summery.get_label());
    print_stdout(dice_states.with_title())?;

    Ok(())
}

pub fn detailed_move_summary(summery: MoveSummary) -> anyhow::Result<()> {
    println!("From Tray:");
    detailed_log_tray(summery.from_tray)?;

    if let Some(tray) = summery.to_tray{
        println!("To Tray:");
        detailed_log_tray(tray)?;
    };

    Ok(())
}


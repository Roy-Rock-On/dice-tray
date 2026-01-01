use cli_table::{Table, WithTitle, format::Justify, print_stdout};
use rust_dice::dice::{Die, DieResult, DieResultType};
use rust_dice::tray::Tray;

#[derive(Table)]
struct DetailedDiceState {
    #[table(title = "Index", justify = "Justify::Center")]
    index: usize,
    #[table(title = "Identity", justify = "Justify::Center")]
    identity: String,
    #[table(title = "Face Count", justify = "Justify::Center")]
    faces_string: String,
    #[table(title = "Current Face", justify = "Justify::Center")]
    current_face_string: String,
    #[table(title = "Result Type", justify = "Justify::Center")]
    result_type_string: String,
    #[table(title = "Result", justify = "Justify::Center")]
    result_string: String,
}

/// Logs the current state of the tray to the console. In table format. Using cli-table crate.
pub fn detailed_log_tray(tray: &dyn Tray) {
    let dice_states: Vec<DetailedDiceState> = tray
        .get_dice()
        .iter()
        .enumerate()
        .map(|(i, die)| DetailedDiceState {
            index: i,
            identity: die.get_id().to_string(),
            faces_string : die.get_face_count().to_string(), 
            current_face_string: die.get_current_face().to_string(),
            result_type_string: die_result_type_to_string(die.as_ref()),
            result_string: die_result_to_string(die.as_ref()),
        })
        .collect(); 

    println!("Showing dice in tray: {}", tray.get_id());
    print_stdout(dice_states.with_title()).unwrap();

    println!(
        "{} = {}",
        tray.get_result_type().to_string(),
        tray.get_result().to_string()
    );
}

/// Converts a DieResult to a String for logging.
fn die_result_to_string(die: &dyn Die) -> String {
    match die.get_result() {
        DieResult::Number(n) => n.to_string(),
        DieResult::String(s) => s.clone(),
        DieResult::None => "None".to_string(),
    }
}

/// Converts a DieResultType to a String for logging.
fn die_result_type_to_string(die: &dyn Die) -> String {
    match die.get_result_type() {
        DieResultType::Face => "Face".to_string(),
        DieResultType::Best => "Best".to_string(),
        DieResultType::Worst => "Worst".to_string(),
        DieResultType::Sum => "Sum".to_string()
    }
}

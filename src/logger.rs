use cli_table::{format::Justify, print_stdout, Cell, Style, Table, WithTitle};
use super::tray::Tray;
use super::dice::{Die, DieResult, DieResultType};

#[derive(Table)]
struct DiceState {
    #[table(title = "Index", justify = "Justify::Center")]
    index: usize,
    #[table(title = "Identity", justify = "Justify::Center")]
    identity: String,
    #[table(title = "Result", justify = "Justify::Center")]
    result_string: String
}

/// Logs the current state of the tray to the console. In table format. Using cli-table crate.
pub fn log_tray(tray: &Tray) {
    let dice_states: Vec<DiceState> = tray.get_dice().iter().enumerate().map(|(i, die)| {
        DiceState {
            index: i,
            identity: die.get_id().to_string(),
            result_string: die_result_to_string(die)
        }
    }).collect();

    print_stdout(dice_states.with_title()).unwrap();
}

/// Converts a DieResult to a String for logging.
fn die_result_to_string(die: &Die) -> String {
    match die.get_result() {
        DieResult::Number(n) => n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_tray() {
        let mut tray = Tray::new();
        let die1 = Die::new(Some("Die1".to_string()), None, 6);
        let die2 = Die::new(Some("Die2".to_string()),None, 6);
        tray.add_die(die1);
        tray.add_die(die2); 
        log_tray(&tray);
    }
}
use regex::Regex;
use std::sync::LazyLock;

pub enum DiceTargets {
    Index(Vec<usize>),
    Label(String),
}

pub fn parse_dice_targets(command: &str) -> Result<Vec<DiceTargets>, String> {
    let split_command = command.split_whitespace();
    let mut targets = Vec::new();

    for part in split_command {
        if part.contains(',') || part.chars().all(|c| c.is_ascii_digit()) {
            let indices: Result<Vec<usize>, _> =
                part.split(',').map(|s| s.trim().parse::<usize>()).collect();

            match indices {
                Ok(idx_vec) if !idx_vec.is_empty() => {
                    targets.push(DiceTargets::Index(idx_vec));
                }
                _ => return Err("Invalid index format".to_string()),
            }
        } else {
            // Treat as label
            targets.push(DiceTargets::Label(part.to_string()));
        }
    }

    if targets.is_empty() {
        Err("No targets found in command.".to_string())
    } else {
        Ok(targets)
    }
}

pub enum CliCommand{
    Exit,
    Help,
    Error(String)
}

impl CliCommand{
    pub fn new(input: &str) -> Self{
        let first_token = match input.split_whitespace().next(){
            Some(first ) => first.to_lowercase(),
            None => return CliCommand::Error("No input found. Please enter a dice-tray command.".to_string())
        };
        
        match first_token.trim(){
            "exit" => CliCommand::Exit,
            "help" => CliCommand::Help,
            _ => CliCommand::Error(format!("{} is not a recognized command. Use 'help' to see a list of valid commands.", first_token))
        }
    }
}

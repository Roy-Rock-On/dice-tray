use regex::Regex;
use anyhow::Error;
use std::fmt::{self, Display, Formatter};

pub enum CliCommand{
    Exit,
    Help,
    Remove(DiceTargets), 
    Error(String)
}

impl CliCommand{
    pub fn new(input: &str) -> Self{
        let mut commands = input.split_whitespace();
        let first_token = match commands.next(){
            Some(first ) => first.to_lowercase(),
            None => return CliCommand::Error("No input found. Please enter a dice-tray command.".to_string())
        };
        
        match first_token.trim(){
            "exit" | "-e" => CliCommand::Exit,
            "help" | "-h" => CliCommand::Help,
            "remove" | "-r" => {
                let other_tokens = commands.collect();
                let targets = match parse_remove_command(other_tokens){
                    Ok(t) => t,
                    Err(e) => return CliCommand::Error(format!("{}", e))
                };
                CliCommand::Remove(targets)
            }
            _ => CliCommand::Error(format!("{} is not a recognized command. Use 'help' to see a list of valid commands.", first_token))
        }
    }
}

fn parse_remove_command(command_parts: Vec<&str>) -> Result<DiceTargets, Error>{
    let mut parts = command_parts.iter().peekable();
    if parts.len() >= 2 {
        return Err(Error::msg("Remove command has too many arguments. Use 'help'see options for dice targeting."));
    }else {
        let targets = parse_dice_targets(
            parts.next().expect("Remove command has no arguments. Use 'help'see options for dice targeting.")
        )?;
        Ok(targets)
    }
}

pub enum DiceTargets {
    All,
    Index(Vec<usize>),
    Label(String),
}

impl Display for DiceTargets{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DiceTargets::All => write!(f, "all"),
            DiceTargets::Index(indices) => write!(f, "indices={:?}", indices),
            DiceTargets::Label(label) => write!(f, "label=\"{}\"", label),
        }
    }
}

fn parse_dice_targets(command: &str) -> Result<DiceTargets, Error> {
    if command.to_lowercase() == "all" {
        return Ok(DiceTargets::All);
    }
    else if command.contains(',') || command.chars().all(|c| c.is_ascii_digit()) {
        let indices: Result<Vec<usize>, _> =
            command.split(',').map(|s| s.trim().parse::<usize>()).collect();

        match indices {
            Ok(id_vec) if !id_vec.is_empty() => {
                return Ok(DiceTargets::Index(id_vec));
            }
            _ => return Err(Error::msg("Invalid index list. Please provide a single index, split integers with ',' or use define a range with '-'.")),
        }
    } else if command.contains('-') {
        let split: Vec<&str> = command.split('-').map(|s| s.trim()).collect();

        if split.len() != 2 {
            return Err(Error::msg(
                "Invalid range format. Please use exactly two integers separated by '-'.",
            ));
        }

        let start = split[0]
            .parse::<usize>()
            .map_err(|_| Error::msg("Range start must be an integer."))?;
        let end = split[1]
            .parse::<usize>()
            .map_err(|_| Error::msg("Range end must be an integer."))?;

        if start > end {
            return Err(Error::msg(
                "Invalid range. Start index must be less than or equal to end index.",
            ));
        }
        let index_range: Vec<usize> = (start..=end).collect();
        return Ok(DiceTargets::Index(index_range));
    }
    else{
        // Treat as label
        return Ok(DiceTargets::Label(command.to_string()));
    }
}


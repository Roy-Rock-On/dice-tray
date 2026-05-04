use anyhow::Error;
use rust_dice::die_allocator::{DiceTargets};

pub enum CliCommand{
    Exit,
    Help,
    Create,
    New,
    Bag,
    Tray(Option<usize>),
    Destroy(DiceTargets),
    Add(DiceTargets, usize),
    Move(DiceTargets, usize),
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
            "bag" | "-b" => CliCommand::Bag,
            "tray" | "-t" => {
                let other_tokens = commands.collect();
                let tray_id = match parse_tray_command(other_tokens) {
                    Ok(id) => id,
                    Err(e) => return CliCommand::Error(format!("{}", e)),
                };
                CliCommand::Tray(tray_id)
            },
            "new" | "-n" => CliCommand::New,
            "create" | "-c" => CliCommand::Create,
            "add" | "-a" => {
                let other_tokens = commands.collect();
                let (targets, tray_id) = match parse_add_command(other_tokens) {
                    Ok(parsed) => parsed,
                    Err(e) => return CliCommand::Error(format!("{}", e))
                };
                CliCommand::Add(targets, tray_id)
            }
            "move" | "-m" => {
                let other_tokens = commands.collect();
                let (targets, tray_id) = match parse_move_command(other_tokens) {
                    Ok(parsed) => parsed,
                    Err(e) => return CliCommand::Error(format!("{}", e))
                };
                CliCommand::Move(targets, tray_id)
            }
            "remove" | "-r" => {
                let other_tokens = commands.collect();
                let targets = match parse_remove_command(other_tokens){
                    Ok(t) => t,
                    Err(e) => return CliCommand::Error(format!("{}", e))
                };
                CliCommand::Remove(targets)
            }
            "destroy" | "delete" | "-d" => {
                let other_tokens = commands.collect();
                let targets = match parse_remove_command(other_tokens){
                    Ok(t) => t,
                    Err(e) => return CliCommand::Error(format!("{}", e))
                };
                CliCommand::Destroy(targets)
            }
            _ => CliCommand::Error(format!("{} is not a recognized command. Use 'help' to see a list of valid commands.", first_token))
        }
    }
}

fn parse_tray_command(command_parts: Vec<&str>) -> Result<Option<usize>, Error> {
    let mut parts = command_parts.iter().peekable();

    match parts.len() {
        0 => Ok(None),
        1 => {
            let tray_id = parts
                .next()
                .expect("Tray command missing ID argument.")
                .parse::<usize>()
                .map_err(|_| Error::msg("Tray ID must be a non-negative integer."))?;
            Ok(Some(tray_id))
        }
        _ => Err(Error::msg("Tray command accepts at most one argument: optional <tray_id>.")),
    }
}

fn parse_add_command(command_parts: Vec<&str>) -> Result<(DiceTargets, usize), Error> {
    let mut parts = command_parts.iter().peekable();
    if parts.len() != 2 {
        return Err(Error::msg("Add command requires exactly two arguments: <targets> <tray_id>. Use 'help' to see dice targeting options."));
    }

    let targets = parse_dice_targets(
        parts.next().expect("Add command missing targets argument.")
    )?;

    let tray_id = parts
        .next()
        .expect("Add command missing tray ID argument.")
        .parse::<usize>()
        .map_err(|_| Error::msg("Tray ID must be a non-negative integer."))?;

    Ok((targets, tray_id))
}

fn parse_move_command(command_parts: Vec<&str>) -> Result<(DiceTargets, usize), Error> {
    let mut parts = command_parts.iter().peekable();
    if parts.len() != 2 {
        return Err(Error::msg("Move command requires exactly two arguments: <targets> <tray_id>. Use 'help' to see dice targeting options."));
    }

    let targets = parse_dice_targets(
        parts.next().expect("Move command missing targets argument.")
    )?;

    let tray_id = parts
        .next()
        .expect("Move command missing tray ID argument.")
        .parse::<usize>()
        .map_err(|_| Error::msg("Tray ID must be a non-negative integer."))?;

    Ok((targets, tray_id))
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


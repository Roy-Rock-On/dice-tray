use crate::dice::Die;
use regex::{Regex, Captures};
use std::sync::LazyLock;

static IDENTITY_FLAG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^\$(\w+(,\w+)*)?").unwrap());
static INDEX_FLAG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^@(\d+(,\d+)*)$").unwrap());
static DICE_NOTATION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(\d*)?[d](\d+)$").unwrap());

/// Enum representing the different types of commands that can be parsed from the CLI.
pub enum DiceTrayCommandType{
    Add,
    Roll,
    Drop,    
    Mod,
    Help,
    Exit
}

/// Struct representing a parsed command from the CLI, including its type and associated string.
pub struct ParsedDiceTrayCommand{
    pub command_type : DiceTrayCommandType,
    pub command_string : Option<String>
}

/// Parses a string of commands from the CLI into a vector of ParsedDiceTrayCommand structs.
pub fn parse_dice_tray_commands(command: &str) -> Vec<ParsedDiceTrayCommand> {
    let mut commands = Vec::new();
    let mut current_command = String::new();
    let mut current_command_type: DiceTrayCommandType = DiceTrayCommandType::Add; // default
    let mut words = command.split_whitespace().peekable();

    while let Some(word) = words.next() {
        if word.starts_with('-') {
            // If we have accumulated a command, save it
            if !current_command.is_empty() {
                commands.push(ParsedDiceTrayCommand {
                    command_type: current_command_type,
                    command_string: Some(current_command.trim().to_string()),
                });
            }
            else{
                // If no command accumulated we can push an empty command
                commands.push(ParsedDiceTrayCommand {
                    command_type: current_command_type,
                    command_string: None,
                });
            }

            if !current_command.is_empty() {

                current_command.clear();
            }

            // Parse the new command flag
            current_command_type = match word {
                "-a" | "-add" => DiceTrayCommandType::Add,
                "-r" | "-roll" => DiceTrayCommandType::Roll,
                "-d" | "-drop" => DiceTrayCommandType::Drop,
                "-m" | "-mod" => DiceTrayCommandType::Mod,
                "-h" | "-help" => DiceTrayCommandType::Help,
                "-e" | "-exit" => DiceTrayCommandType::Exit,
                _ => DiceTrayCommandType::Roll, // default for unknown flags
            };
        } else {
            // Add word to current command
            if !current_command.is_empty() {
                current_command.push(' ');
            }
            current_command.push_str(word);
        }
    }

    commands.push(ParsedDiceTrayCommand {
        command_type: current_command_type,
        command_string: Some(current_command.trim().to_string()),
    });
    commands
}

/// Parses a roll command. Checking for identity flags and dice notation. Then returns a vector of dice to be added to the tray.
pub fn parse_add_command(command: Option<&str>) -> Option<Vec<Die>> {
    match command {
        Some(cmd) => {
            let mut identity_flag: Option<Vec<String>> = None;
            let mut split_command = cmd.split_whitespace();
            let mut dice_to_add: Vec<Die> = Vec::new();

            while let Some(command) = split_command.next() {
                if let Some(captured_id_flag) = IDENTITY_FLAG_REGEX.captures(command) {
                    identity_flag = parse_identity_flag(&captured_id_flag);
                }
                else if let Some(captured_dice_expression) = DICE_NOTATION_REGEX.captures(command) {
                    if let Some((count, faces)) = parse_dice_notation(&captured_dice_expression) {
                        for _ in 0..count {
                            let identity = match &identity_flag {
                                Some(ids) if ids.len() > 0 => Some(ids[0].clone()), // Use the first identity if multiple provided
                                _ => None
                            };
                            println!("Adding die with {} faces and identity {:?} ", faces, identity_flag);
                            let die = Die::new(identity, faces);
                            dice_to_add.push(die);
                        }
                    }
                }
            }
        Some(dice_to_add)
        },
        None => None
    }
}

/// Parses a roll command, checking for identity and index flags. If none are found, returns None and all dice in the tray are rolled.
pub fn parse_roll_command(roll_command: Option<&str>) -> (Option<Vec<String>>, Option<Vec<usize>>) {
    match roll_command {
        Some(cmd) => {
            let mut identity_flags: Option<Vec<String>> = None;
            let mut index_flags: Option<Vec<usize>> = None;
            let mut split_command = cmd.split_whitespace();
            while let Some(command) = split_command.next() {
                if let Some(captured_id_flag) = IDENTITY_FLAG_REGEX.captures(command) {
                    identity_flags = parse_identity_flag(&captured_id_flag);
                }
                else if let Some(captured_index_flag) = INDEX_FLAG_REGEX.captures(command) {
                    index_flags = parse_index_flag(&captured_index_flag);
                }
            }
            (identity_flags, index_flags)
        }
        None => (None, None)
    }
}

fn parse_identity_flag(captures: &Captures) -> Option<Vec<String>> {
    match captures.get(1) {
        Some(m) => Some(m.as_str().split(',').map(|s| s.to_string()).collect()),
        None => None
    }
}

fn parse_index_flag(captures: &Captures) -> Option<Vec<usize>> {
    match captures.get(1) {
        Some(m) => {
            let indices: Vec<usize> = m.as_str()
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            Some(indices)
        },
        None => None
    }
}

fn parse_dice_notation(captures: &Captures) -> Option<(u32, u32)> { 
    let count = match captures.get(1) {
        Some(m) => {
            if m.as_str().is_empty() {
                1
            } else {
                m.as_str().parse::<u32>().unwrap_or(1)
            }
        },
        None => 1
    };

    let faces = match captures.get(2) {
        Some(m) => m.as_str().parse::<u32>().unwrap_or(6),
        None => 6
    };

    Some((count, faces))
}

/// Parses a drop command, checking for identity and index flags. Returns the parsed flags.
pub fn parse_drop_command(drop_command: Option<&str>) -> (Option<Vec<String>>, Option<Vec<usize>>) {
    match drop_command {
        Some(cmd) => {
            let mut identity_flags: Option<Vec<String>> = None;
            let mut index_flags: Option<Vec<usize>> = None;
            let mut split_command = cmd.split_whitespace();
            while let Some(command) = split_command.next() {
                if let Some(captured_id_flag) = IDENTITY_FLAG_REGEX.captures(command) {
                    identity_flags = parse_identity_flag(&captured_id_flag);
                }
                else if let Some(captured_index_flag) = INDEX_FLAG_REGEX.captures(command) {
                    index_flags = parse_index_flag(&captured_index_flag);
                }
            }
            (identity_flags, index_flags)
        }
        None => (None, None)
    }
}
use regex::{Regex};
use std::sync::LazyLock;

static DICE_NOTATION_REGEX: LazyLock<Regex> =  LazyLock::new(|| Regex::new(r"(?i)^(\d*)?[d](\d+)$").unwrap());

pub fn parse_dice_notation(command: &str) -> Result<Vec<(u32, u32)>, String> {
    let split_command = command.split_whitespace();
    let mut dice_vec = Vec::new();

    for part in split_command {
        if let Some(captures) = DICE_NOTATION_REGEX.captures(part) {
            let count = match captures.get(1) {
                Some(m) => {
                    if m.as_str().is_empty() {
                        1
                    } else {
                        m.as_str().parse::<u32>().unwrap_or(1)
                    }
                }
                None => 1,
            };

            let faces = match captures.get(2) {
                Some(m) => m.as_str().parse::<u32>().unwrap_or(6),
                None => 6,
            };

            dice_vec.push((count, faces));
        }
    }

    if dice_vec.is_empty() {
        Err("No dice notation found in dice command.".to_string())
    } else {
        Ok(dice_vec)
    }
}

pub enum DiceTargets{
    Index(Vec<usize>),
    Label(String)
}

pub fn parse_dice_targets(command: &str) -> Result<Vec<DiceTargets>, String>{
    let split_command = command.split_whitespace();
    let mut targets = Vec::new();
        
    for part in split_command {
        if part.contains(',') || part.chars().all(|c| c.is_ascii_digit()) {
            let indices: Result<Vec<usize>, _> = part
                .split(',')
                .map(|s| s.trim().parse::<usize>())
                .collect();
            
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


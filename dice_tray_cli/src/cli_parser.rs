use rust_dice::dice_builders::{new_num_die};
use regex::{Regex};
use std::sync::LazyLock;

static DICE_NOTATION_REGEX: LazyLock<Regex> =  LazyLock::new(|| Regex::new(r"(?i)^(\d*)?[d](\d+)$").unwrap());

pub fn parse_dice_notation(command: &str) -> Result<(u32, u32), String> {
    if let Some(captures) = DICE_NOTATION_REGEX.captures(command){
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

       Ok((count, faces))
    }
    else{
        Err("No dice notation found in dice command.".to_string())
    }
}


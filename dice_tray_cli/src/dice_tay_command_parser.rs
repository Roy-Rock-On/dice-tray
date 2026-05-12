use std::fmt::{Display, format};

use anyhow::{Error, Context, Result, anyhow};

#[derive(Debug, Clone)]
enum DiceTrayClause{
    Label(String),
    Count(u32),
    Index(Vec<u32>),
    Roll,
    Add,
    Remove,
    Die,
    Tray,
    Bag,
    Profile,
    From,
    To,
    Create,
    Delete,
    Move,
    Save,
    Load,
    Unrecognized(String)
}

impl DiceTrayClause{
    fn from_str(command: &str) -> Self {
        let token = command.trim().to_lowercase().to_string();

        match token.as_str(){
            "roll" => DiceTrayClause::Roll,
            "add" => DiceTrayClause::Add,
            "remove" => DiceTrayClause::Remove,
            "die" | "dice" => DiceTrayClause::Die,
            "tray" => DiceTrayClause::Tray,
            "bag" => DiceTrayClause::Bag,
            "profile" => DiceTrayClause::Profile,
            "from" => DiceTrayClause::From,
            "to" => DiceTrayClause::To,
            "create" | "new" => DiceTrayClause::Create,
            "delete" | "destroy" => DiceTrayClause::Delete,
            "move" => DiceTrayClause::Move,
            "save" => DiceTrayClause::Save,
            "load" => DiceTrayClause::Load,

            _ => {
                match parse_complex_token(command){
                    Ok(clause) => clause,
                    Err(e) => DiceTrayClause::Unrecognized(format!("Could not parse clause with errors {:?}", e))
                }
            }
        }
    }
}

fn parse_complex_token(token: &str) -> anyhow::Result<DiceTrayClause> {
    if let Ok(num) = token.parse::<u32>(){
        return Ok(DiceTrayClause::Count(num));
    }

    let mut token_view = token.chars().clone();
    match token_view.next(){
        Some(c) => {
            if c == '@'{
                return parse_indecies(token_view.as_str());
            }
        },
        None => ()
    }

    if token.chars().all(|c| {
        (c.is_alphanumeric() || c.is_ascii_punctuation()) 
        && !c.is_whitespace() 
        && !c.is_control()
    }){
        return Ok(DiceTrayClause::Label(token.to_string()));
    }
   
    Ok(DiceTrayClause::Unrecognized(format!("Complex token unrecognized {}", token)))
}

fn parse_indecies(token: &str) -> anyhow::Result<DiceTrayClause>{
    if token.contains(',') || token.chars().all(|c| c.is_ascii_digit()) {
        let indices: Vec<u32> = token.split(',')
            .map(|s| s.trim().parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .context("Failed to parse indicies")?;

        Ok(DiceTrayClause::Index(indices))
    }
    else if token.contains('-') {
            let split: Vec<&str> = token.split('-').map(|s| s.trim()).collect();

            if split.len() != 2 {
                return Ok(DiceTrayClause::Unrecognized(format!("Unrecognized token while parsing token {} for indicies.", token)));
            }

            let start = split[0]
                .parse::<u32>()
                .map_err(|_| Error::msg("Range start must be an integer."))?;
            let end = split[1]
                .parse::<u32>()
                .map_err(|_| Error::msg("Range end must be an integer."))?;

            if start > end {
                return Ok(DiceTrayClause::Unrecognized(format!("Unrecognized token while parsing token {} for indicies.", token)))
            }
            let index_range: Vec<u32> = (start..=end).collect();
            return Ok(DiceTrayClause::Index(index_range));
    }
    else{
        return Ok(DiceTrayClause::Unrecognized(format!("Unrecognized token while parsing token {} for indicies", token)))
    }
}

pub fn get_command_string() -> String{
    use std::io::{stdout, stdin};
    use std::io::Write;

    let mut input = String::new();
    stdout().flush().expect("failed to flush stdout");
    input.clear();

    let _ = stdin().read_line(&mut input);

    let command = input.trim();
    command.to_string()
}

enum DiceTrayCommand {
    NewDie(NewDieArgs),
    DeleteDie(usize),
    NewTray(String),
    DeleteTray(String),
    AddDice(AddDiceArgs),
    MoveDice(MoveDiceArgs),
    RollDice(RollDiceArgs),
    ShowTray(String),
    ShowDiceBag
}

struct NewDieArgs{
    pub label: String,
    pub faces: u32,
    pub varience: Option<u32>
}

struct AddDiceArgs{
   pub should_roll: bool,
   pub number: u32,
   pub dice_targets: Vec<usize>,
   pub tray_target: Option<String>
}

struct MoveDiceArgs{
    pub should_roll: bool,
    pub dice_targets: Vec<usize>,
    pub from_tray: Option<String>,
    pub to_tray: Option<String>
}

struct RollDiceArgs{
    pub dice_targets: Vec<usize>,
    pub tray_target: Option<String>
}

fn parse_clauses(clauses : Vec<DiceTrayClause>) -> Vec<DiceTrayCommand> {
    let mut commands: Vec<DiceTrayCommand> = Vec::new();
    let mut context: Vec<DiceTrayClause> = Vec::new();
    
    let clauses_iter = 

    loop {
        let next_clause = clauses.next();





        if clauses.is_empty() { break; }
    }

    commands
}

#[cfg(test)]
#[test]

fn test_parse_tokens(){
    let commands = get_command_string();
    let trimed = commands.trim();
    let clauses : Vec<DiceTrayClause> = trimed
        .split_whitespace()
        .into_iter()
        .map(|c| DiceTrayClause::from_str(c))
        .collect();

    println!("From Command =");
    println!("{}", commands);

    println!();
    println!("Found clauses = ");
    println!("{:?}", clauses);
}


/*
match clause {
    DiceTrayClause::Count(num) => (),
    DiceTrayClause::Index(indecies) => (),
    DiceTrayClause::Label(label) => (),
    DiceTrayClause::Add => (),
    DiceTrayClause::Bag => (),
    DiceTrayClause::Create => (),
    DiceTrayClause::Delete => (),
    DiceTrayClause::Die => (),
    DiceTrayClause::From => (),
    DiceTrayClause::Load => (),
    DiceTrayClause::Move => (),
    DiceTrayClause::Profile => (),
    DiceTrayClause::Roll => (),
    DiceTrayClause::Remove => (),
    DiceTrayClause::Tray => (),
    DiceTrayClause::To => (),
    DiceTrayClause::Save => (),
    DiceTrayClause::Unrecognized(message) => ()
}
*/


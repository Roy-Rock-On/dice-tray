use std::{fmt::{Display, format}, path::PathBuf};

use anyhow::{Error, Context, Result, bail};
use clap::parser::Indices;
use std::path::Path;

const STD_WEIGHT: u32 = 100;

fn process_command(input : &str) -> anyhow::Result<Vec<DiceTrayCommand>> {
    let mut commands : Vec<DiceTrayCommand> = Vec::new();
    let input_vector : Vec<&str> = input.trim().split_ascii_whitespace().collect();
    
    let normalized_input = input.trim().to_lowercase().clone();
    let normalized_vector : Vec<&str> = normalized_input.split_whitespace().collect();
    let length = normalized_vector.len();

    let command_boundaries = get_command_boundaries(&normalized_input);

    if command_boundaries.is_empty(){
        bail!("No commands found within the input string = '{}'", input);
    } else {

        let mut boundaries_iter = command_boundaries.iter().peekable();
        
        while let Some(start_slice) = boundaries_iter.next() {
            let end_slice = boundaries_iter.peek().copied().unwrap_or(&length);

            // Remove the '&' from the start. .get() already returns a reference.
            let input_slice = input_vector.get(*start_slice..*end_slice)
                .context("Failed to slice input while parsing commands")?;
                
            let normalized_slice = normalized_vector.get(*start_slice..*end_slice)
                .context("Failed to slice input while parsing commands")?;

            // Most Rust functions expect &[T], so we pass input_slice directly
            match parse_command(input_slice, normalized_slice) {
                Ok(command) => commands.push(command),
                Err(e) => bail!("Failed to parse command at index {}: {}", start_slice, e)
            }
        }
    }
    Ok(commands)
}

fn parse_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    if let Some(token) = normalized_slice.iter().peekable().peek(){
        match token.trim(){
            "roll" => parse_add_command(true, input_slice, normalized_slice),
            "add" => parse_add_command(false, input_slice, normalized_slice),
            "move" => parse_move_command(true, input_slice, normalized_slice),
            "remove" | "place" => parse_move_command(true, input_slice, normalized_slice),
            "create" | "new" => parse_create_command(input_slice, normalized_slice),
            "delete" | "destroy" => parse_destroy_command(input_slice, normalized_slice),
            "save" => parse_save_command(input_slice, normalized_slice),
            "load" => parse_load_command(input_slice, normalized_slice),
            "show" => parse_show_command(input_slice, normalized_slice),
            _ => bail!("Token {} not recognized while parsing command.", token)
        }
    }
    else{
        bail!("Slice empty when parsing command.")
    }
}

fn parse_add_command(should_roll: bool, input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    let mut args = AddDiceArgs{
        should_roll,
        number: 1,
        dice_targets: DiceTargets::None,
        tray_target: None
    };
    let mut normal_iter = normalized_slice.iter().enumerate().peekable();
    let _ = normal_iter.next(); //Clear the initial command.

    while let Some((index, token)) = normal_iter.next(){
        let parsed_token = parse_complex_token(token)?;
        match parsed_token {
            ComplexToken::Number(num) => args.number = num,
            ComplexToken::Indices(targets) => args.dice_targets = DiceTargets::Indices(targets),
            ComplexToken::Die =>{
                if let Some((next_index, next_token)) = normal_iter.next(){
                    match parse_complex_token(*next_token)?{
                        ComplexToken::Indices(indices) => args.dice_targets = DiceTargets::Indices(indices),
                        ComplexToken::Other => {
                            if let Some(token) = input_slice.get(next_index){
                                args.dice_targets = DiceTargets::Label(parse_as_label(token)?);
                            }                            
                        }
                        _ => bail!("'die' token must be followed by valid die targets. Found '{}' instead.", next_token)
                    }
                }
            },
            ComplexToken::Tray => {
                if let Some((next_index, next_token)) = normal_iter.next(){
                    if let Some(token) = input_slice.get(next_index){
                        args.tray_target = Some(parse_as_label(token)?);
                    }
                    else{
                        bail!("'tray' token must be followed by valid tray Label. Found '{}' instead.", next_token)
                    }                            
                }
            },
            ComplexToken::Filler => (),
            _ => {
                if let Some(input_token) = input_slice.get(index){
                    let label = parse_as_label(input_token)?;
                    if args.dice_targets == DiceTargets::None {
                        args.dice_targets = DiceTargets::Label(label);
                    }
                }
            }
        }
    }

    Ok(DiceTrayCommand::AddDice(args))
}

fn parse_create_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    let mut normal_iter = normalized_slice.iter().enumerate().peekable();
    let _ = normal_iter.next(); //Get the initial value out of the iter. It will always be the original command. 

    println!("Checking the first token of the create command");
    let first_token = match normal_iter.peek(){
        Some((_index, token)) => **token,
        None => bail!("'new' command has no content.")
    };

    //If making a new tray
    if first_token == "tray" {
        let _ = normal_iter.next();
        let token_index: usize = match normal_iter.next(){
            Some((index, _token)) => index,
            None => bail!("'new tray' command does not contain a label. All trays require a unique label.")
        };
        let label_token = match input_slice.get(token_index){
            Some(token) => token,
            None => bail!("'new tray' command cannot find a token at index {} in the input slice. This really shouldn't be possible.", token_index)
        };
        let tray_label = parse_as_label(*label_token)?;
        Ok(DiceTrayCommand::NewTray(tray_label))
    } 
    else //We make a new die.
    {
        let mut args = CreateDieArgs{
                label : None,
                faces : None,
                variance : None
        };
            
        while let Some((index, token)) = normal_iter.next(){
            let parsed_token = parse_complex_token(token)?;
            match parsed_token{
                ComplexToken::Other => {
                    if let Some(token) = input_slice.get(index){
                        args.label = Some(parse_as_label(token)?);
                    }
                },
                ComplexToken::Number(num) => args.faces = Some(num),
                ComplexToken::Var =>{
                    if let Some((index, next_token)) = normal_iter.next(){
                        match parse_complex_token(next_token)?{
                            ComplexToken::Number(num) => args.variance = Some(num),
                            _ => bail!("'var' token must be followed by a number to map face variance. {} found instead.", next_token)
                        }
                    }
                    else {
                        bail!("'var' token must be followed by a number to map face variance. Nothing found.");
                    }
                }
                _ => ()
            }
        }
        Ok(DiceTrayCommand::NewDie(args))
    } 
}

fn parse_destroy_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    let mut normal_iter = normalized_slice.iter().enumerate().peekable();
    let _ = normal_iter.next(); //Get the initial value out of the iter. It will always be the original command. 

    println!("Checking the first token of the create command");
    let first_token = match normal_iter.peek(){
        Some((index, token)) => **token,
        None => bail!("'new' command has no content.")
    };

    //If destroying a tray.
    if first_token == "tray" {
        let _ = normal_iter.next();
        let token_index: usize = match normal_iter.next(){
            Some((index, _token)) => index,
            None => bail!("'delete tray' command does not contain a label. All trays require a unique label.")
        };
        let label_token = match input_slice.get(token_index){
            Some(token) => token,
            None => bail!("'delete tray' command cannot find a token at index {} in the input slice. This really shouldn't be possible.", token_index)
        };
        let tray_label = parse_as_label(*label_token)?;
        Ok(DiceTrayCommand::DeleteTray(tray_label))
    } 
    else //We destroy some dice.
    {
        while let Some((index, token)) = normal_iter.next(){
            let parsed_token = parse_complex_token(token)?;
            match parsed_token{
                ComplexToken::Indices(indices) => return Ok(DiceTrayCommand::DeleteDice(DiceTargets::Indices(indices))),
                ComplexToken::Other => {
                    if let Some(token) = input_slice.get(index){
                        let label = parse_as_label(token)?;
                        return Ok(DiceTrayCommand::DeleteDice(DiceTargets::Label(label)));
                    }
                },
                _ => ()
            }
        }
        bail!("'destroy dice' command did not find a dice target to destroy.");
    }     
}

fn parse_load_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    use std::path::Path;
    
    let mut input_iter = input_slice.iter();
    let _ = input_iter.next(); //Clear command token.

    if let Some(path_token) = input_iter.next(){
        let load_path = Path::new(path_token);
        if load_path.is_file() {
            return Ok(DiceTrayCommand::Load(Some(PathBuf::from(load_path))));
        }
        else {
            bail!("'load' command found no file at given path: {}", path_token);
        }
    }else{
        return Ok(DiceTrayCommand::Load(None));
    }
}

fn parse_save_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    use std::path::Path;
    
    let mut input_iter = input_slice.iter();
    let _ = input_iter.next(); //Clear command token.

    if let Some(path_token) = input_iter.next(){
        let save_path = Path::new(path_token);
        if save_path.is_file() {
            return Ok(DiceTrayCommand::Save(Some(PathBuf::from(save_path))));
        }
        else {
            bail!("'save' command found no file at given path: {}", path_token);
        }
    }else{
        return Ok(DiceTrayCommand::Save(None));
    }
}

fn parse_move_command(should_roll: bool, input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    todo!();
}

fn parse_show_command(input_slice : &[&str], normalized_slice : &[&str]) -> anyhow::Result<DiceTrayCommand>{
    todo!();
}

fn get_command_boundaries(command : &str) -> Vec<usize> {
    let mut boundaries: Vec<usize> = Vec::new();
    for (index, clause) in command.split_whitespace().enumerate() {
        match clause.trim() {
            "roll" |
            "add" |
            "remove" |
            "move" |
            "create" | "new" |
            "delete" | "destroy" |
            "save" |
            "load" |
            "show" => boundaries.push(index),
            _ => ()
        }
    }
    boundaries
}

#[derive(Debug, PartialEq)]
enum DiceTargets{
    Indices(Vec<usize>),
    Label(String),
    None
}

#[derive(Debug)]
enum ComplexToken {
    Number(u32),
    Indices(Vec<usize>),
    Tray,
    Die,
    Var,
    Filler,
    Other
}

fn parse_complex_token(token: &str) -> anyhow::Result<ComplexToken> {
    if let Some(token) = match token{
        "die" => Some(ComplexToken::Die),
        "tray" => Some(ComplexToken::Tray),
        "var" => Some(ComplexToken::Var),
        "to" | "from" | "at" | "with" => Some(ComplexToken::Filler),
        _ => None
    }{
        return Ok(token);
    }

    if let Ok(num) = token.parse::<u32>(){
        return Ok(ComplexToken::Number(num));
    }

    let mut token_view = token.chars().clone();

    match token_view.next(){
        Some(c) => {
            if c == '@'{
                let indices = parse_indices(token_view.as_str())?;
                return Ok(ComplexToken::Indices(indices));
            }
        },
        None => ()
    }

    Ok(ComplexToken::Other)
}

fn parse_as_label(token: &str) -> anyhow::Result<String>{
    if token.chars().all(|c| {
        (c.is_alphanumeric() || c.is_ascii_punctuation()) 
        && !c.is_whitespace() 
        && !c.is_control()
    }){
        return Ok(token.to_string());
    }
    else{
        bail!("Could not parse token: '{}' as a label. It may contain invalid characters.", token);
    }
}

fn parse_indices(token: &str) -> anyhow::Result<Vec<usize>>{
    if token.contains(',') || token.chars().all(|c| c.is_ascii_digit()) {
        let indices: Vec<usize> = token.split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .context("Failed to parse indicies")?;

        Ok(indices)
    }
    else if token.contains('-') {
            let split: Vec<&str> = token.split('-').map(|s| s.trim()).collect();

            if split.len() != 2 {
                bail!("Unrecognized token while parsing token {} for indicies.", token);
            }

            let start = split[0]
                .parse::<usize>()
                .map_err(|_| Error::msg("Range start must be an integer."))?;
            let end = split[1]
                .parse::<usize>()
                .map_err(|_| Error::msg("Range end must be an integer."))?;

            if start > end {
                bail!("Unrecognized token while parsing token {} for indicies.", token);
            }
            let index_range: Vec<usize> = (start..=end).collect();
            return Ok(index_range);
    }
    else{
        bail!("Unrecognized token while parsing token {} for indicies", token);
    }
}

pub fn get_command_string() -> String{
    use std::io::{stdout, stdin};
    use std::io::Write;

    let mut input =
 String::new();
    stdout().flush().expect("failed to flush stdout");
    input.clear();

    let _ = stdin().read_line(&mut input);

    let command = input.trim();
    command.to_string()
}

#[derive(Debug)]
enum DiceTrayCommand {
    NewDie(CreateDieArgs),
    DeleteDice(DiceTargets),
    NewTray(String),
    DeleteTray(String),
    AddDice(AddDiceArgs),
    MoveDice(MoveDiceArgs),
    RollDice(RollDiceArgs),
    ShowTray(String),
    ShowDiceBag,
    Save(Option<PathBuf>),
    Load(Option<PathBuf>)
}

#[derive(Debug)]
struct CreateDieArgs{
    pub label: Option<String>,
    pub faces: Option<u32>,
    pub variance: Option<u32>
}

#[derive(Debug)]
struct AddDiceArgs{
   pub should_roll: bool,
   pub number: u32,
   pub dice_targets: DiceTargets,
   pub tray_target: Option<String>
}

#[derive(Debug)]
struct MoveDiceArgs{
    pub should_roll: bool,
    pub dice_targets: DiceTargets,
    pub from_tray: Option<String>,
    pub to_tray: Option<String>
}

#[derive(Debug)]
struct RollDiceArgs{
    pub dice_targets: DiceTargets,
    pub tray_target: Option<String>
}


#[cfg(test)]
#[test]
fn test_parser() -> anyhow::Result<()> {
    let input = "new tray MAIN new die 6 FIREBALL var 100 roll 8 @0-10".to_string();

    let commands = process_command(&input).unwrap();
    println!("Found tray commands = {}", commands.len());

    for command in commands{
        println!("{:?}", command)
    } 

    Ok(())
}

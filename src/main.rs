use std::io;
use clap::{arg, Parser};
use dice_tray::tray::{self, Tray};
use dice_tray::logger::log_tray;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct TrayArgs {
    /// Rolls dice and adds them the tray.
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    roll : Vec<String>,

    ///Clears the tray of all dice.
    #[arg(short, long, default_value_t = false)]
    clear : bool,

    ///Shakes the tray to re-roll all dice.
    #[arg(short, long, default_value_t = false)]
    shake : bool,

    ///Exits the applicaiton.
    #[arg(short, long, default_value_t = false)]
    exit : bool,

    ///Names the die at the specified index. Or that have the specified identiy.
    #[arg(short, long, num_args = 2)]
    name : Vec<String> 
}

static DICE_NOTATION_REGEX: &str = r"(?i)^(?:(\d*)?[dD](\d+))$";

fn main() {
    let mut active_tray = Tray::new();
    let mut tray_iterations = 0;
    println!("Welcome to Dice Tray!");
    log_tray(&active_tray);
    loop {
        println!("Enter commands (or use --help for options):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_args: Vec<&str> = input.trim().split_whitespace().collect();
        
        let args = match TrayArgs::try_parse_from(std::iter::once("dice-tray").chain(input_args)) {
            Ok(args) => args,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        
        // Handle clearing the tray
        if args.clear {
            println!("Clearing tray...");
            active_tray.clear();
        }

        // Handle rolling dice
        if !args.roll.is_empty() {
            for notation in args.roll {
                if let Some((count, faces)) = parse_dice_notation(&notation) {
                    let new_dice = dice_tray::dice::new_dice_set(count, faces);
                    for mut die in new_dice {
                        active_tray.add_die(die);
                    }
                }
            }
        }

        // Handle shaking the tray
        if args.shake {
            println!("Shaking tray...");
            active_tray.roll_all();
        }

        // Log the current state of the tray

        println!("Count of dice in tray: {}", active_tray.get_dice().len());
        log_tray(&active_tray);

        // Handle exiting the application
        if args.exit {
            break;
        }
    }
}

fn parse_dice_notation(notation: &str) -> Option<(u32, u32)> {
    let re = Regex::new(DICE_NOTATION_REGEX).unwrap();
    if let Some(captures) = re.captures(notation) {
        let count = captures.get(1).map_or("1", |m| m.as_str()).parse::<u32>().ok()?;
        let faces = captures.get(2)?.as_str().parse::<u32>().ok()?;
        Some((count, faces))
    } else {
        None
    }
}

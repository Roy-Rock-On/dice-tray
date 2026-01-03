mod app;
mod cli_dice_allocator;
mod cli_dice_tray;
mod cli_parser;
mod logger;

use cli_parser::{parse_dice_notation, parse_dice_targets};

use app::CliDiceTrayApp;

use clap::{Parser, Subcommand};
use rust_dice::dice::DieResultType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    ///If true, a summary of all trays will be generated. If not, only the target tray will be shown.
    verbose: bool,

    #[arg(long, short)]
    ///A tray target. If no tray with the given ID is available the default "Main" tray will be targeted.
    tray: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///Resets dice_tray_cli by clearing all trays and dice. Cannot be undone (at least for now)
    Reset,
    ///Deletes the target tray and all the dice in it. If no target tray is provided using the --tray option nothing happens. The main tray can't be deleted.
    Delete,
    //Dice commands
    /// Adds dice to a tray. Usage: add -t "fireball" "8d6 d2" : would roll 8 six-sided dice and a 2 sided-die to "fireball" tray. Targeting a tray that dosen't exist will create a new tray.
    Add {
        #[arg(short, long)]
        ///Optional result type. Current result types supported are: 'f' = the die's current face, 'b' = the best result the die has rolled, 'w' = the worst result the die has rolled, 'e' = sum of all results.
        result_type: Option<char>,
        ///Basic dice notation seperated by whitespace i.e. "4d8" = four eight-sided dice, "2d4 d14" = 2 four-sided dice, and a 14 sided-die.
        dice_command: String,
    },
    ///Drop removes dice from the tray based on the provided dice tragets. If no targets are provided the tray is cleared of all dice.
    Drop {
        ///Optional dice targets, either by label or by index. If no targets are provided all dice in the target tray will be removed.
        dice_targets: Option<String>,
    },
    ///Rolls the dice in the target tray at the provided dice targets(i.e. by index "0,4,6" or by id "d100").
    Roll {
        #[arg(short, long)]
        ///Optional result type. Current result types supported are: 'f' = the die's current face, 'b' = the best result the die has rolled, 'w' = the worst result the die has rolled, 'e' = sum of all results.
        result_type: Option<char>,
        ///Optional dice targets, either by label or by index. If no targets are provided all dice in the target tray will be rolled.
        dice_targets: Option<String>,
    },
}

fn main() {
    let mut app = CliDiceTrayApp::new();
    app.init();
    let cli = Cli::parse();
    let tray_id: Option<&str> = cli.tray.as_deref();

    match &cli.command {
        Some(Commands::Reset) => {
            app.reset();
        }
        Some(Commands::Delete) => {
            app.delete_tray(tray_id);
        }
        Some(Commands::Drop { dice_targets }) => match dice_targets {
            Some(target_string) => {
                if let Ok(targets) = parse_dice_targets(target_string) {
                    match app.drop_at_targets(tray_id, targets) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to remove dice at provided targets with error {}", e)
                        }
                    }
                }
            }
            None => {
                app.drop_all(tray_id);
            }
        },
        Some(Commands::Add {
            result_type,
            dice_command,
        }) => {
            let result_type_unpacked = find_result_type(*result_type);
            println!("Result type is = {:?}", result_type_unpacked);
            if let Ok(raw_dice) = parse_dice_notation(dice_command) {
                raw_dice.iter().for_each(|dice| {
                    app.add_dice_from_raw(tray_id, dice.0, dice.1, result_type_unpacked);
                });
            }
        }
        Some(Commands::Roll {
            result_type,
            dice_targets,
        }) => {
            let result_type = find_result_type(*result_type);
            match dice_targets {
                Some(target_string) => {
                    if let Ok(targets) = parse_dice_targets(target_string) {
                        match app.roll_at_targets(tray_id, targets, result_type) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to roll dice at provided targets with error {}", e)
                            }
                        }
                    }
                }
                None => {
                    app.roll_all(tray_id, result_type);
                }
            }
        }
        None => {
            println!("No commands found!")
        }
    };

    if cli.verbose {
        app.show_all_trays();
    } else {
        app.show_tray(tray_id);
    }
    app.close();
}

fn find_result_type(c: Option<char>) -> Option<DieResultType> {
    match c {
        Some('f') => Some(DieResultType::Face),
        Some('b') => Some(DieResultType::Best),
        Some('w') => Some(DieResultType::Worst),
        Some('e') => Some(DieResultType::Sum),
        _ => None,
    }
}

mod cli_parser;
mod cli_dice_tray;
mod cli_dice_allocator;
mod logger;
mod app;

use cli_parser::{DiceTargets, parse_dice_targets, parse_dice_notation};

use app::CliDiceTrayApp;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(long, short)]
    verbose: bool,

    #[arg(long, short)]
    tray: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands{
    ///Resets dice_tray_cli by clearing all trays and dice. Cannot be undone (at least for now)
    Reset,
    //Dice commands
    /// Adds dice to a tray. Usage: add -t "fireball" "8d6 d2" : would roll 8 six-sided dice and a 2 sided-die to "fireball" tray.
    Add{
        ///Basic dice notation seperated by whitespace i.e. "4d8" = four eight-sided dice, "2d4 d14" = 2 four-sided dice, and a 14 sided-die. 
        dice_command: String
    },
    ///
    Remove{
        ///A basic dice string i.e. "#d#". Many dice strings can be seperated by whitespace to roll many dice at once.  
        dice_targets: String
    },
    Roll {
        ///Optional dice targets, either by label or by index. If no targets are provided all dice in the target tray will be rolled. 
        dice_targets: Option<String>
    }

}

fn main(){
    let mut app =  CliDiceTrayApp::new();
    app.init();
    let cli = Cli::parse();
    let tray_id : Option<&str> = cli.tray.as_deref();

    match &cli.command {
        Some(Commands::Remove { dice_targets}) => {
            if let Ok(targets) = parse_dice_targets(dice_targets){
                match app.remove_at_targets(tray_id, targets){
                    Ok(_) => {},
                    Err(e) => {println!("Failed to remove dice at provided targets with error {}" , e)}
                }
            }
        },
        Some(Commands::Reset) => {
            app.reset();
        },
        Some(Commands::Add { dice_command}) => {
            if let Ok(raw_dice) = parse_dice_notation(dice_command){
                raw_dice.iter().for_each(|dice|{
                    app.add_dice_from_raw(tray_id,dice.0, dice.1);
                });
            }
        },
        Some(Commands::Roll { dice_targets}) => {
            match dice_targets{
                Some(target_string) => {
                    if let Ok(targets) = parse_dice_targets(target_string){
                        match app.roll_at_targets(tray_id, targets){
                            Ok(_) => {},
                            Err(e) => {println!("Failed to roll dice at provided targets with error {}" , e)}
                        }
                    }
                }
                None => {
                    app.roll_all(tray_id);
                }
            }
        },
        None => {println!("No commands found!")}
    };

    app.show_tray(tray_id);
    app.close();
}

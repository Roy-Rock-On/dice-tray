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

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands{
    ///Resets dice_tray_cli by clearing all trays and dice. Cannot be undone, at least for now.
    Reset,
    //Dice commands
    Add{
        #[arg(short, long)]
        ///Each tray has a unique string ID "Tray ID". When adding, if no tray is present with the given ID then a new tray will be created. 
        tray: Option<String>,
        ///A basic dice string i.e. "#d#". Many dice strings can be seperated by whitespace to roll many dice at once.  
        dice_command: String
    },
    Remove{
        #[arg(short, long)]
        ///Each tray has a unique string ID "Tray ID". If no tray is provided dice will be removed from the default tray.
        tray: Option<String>,
        ///A basic dice string i.e. "#d#". Many dice strings can be seperated by whitespace to roll many dice at once.  
        dice_command: String
    },
    Roll {
        #[arg(short, long)]
        ///An optional tray ID for the dice you want to roll. If no tray is provided dice will be rolled in the default tray.
        tray: Option<String>,
        ///Optional dice targets, either by label or by index. 
        targets: Option<String>
    }

}

fn main(){
    let mut app =  CliDiceTrayApp::new();
    app.init();
    let cli = Cli::parse();
    if cli.verbose{
        println!("cli is verbose!");
    }
    match &cli.command {
        Some(Commands::Remove {
            tray,
            dice_command
        }) => {
            match tray{
               Some(tray_string) => {
                app.target_tray(tray_string);
            }
               None => {} 
            };
            if let Ok(targets) = parse_dice_targets(dice_command){
            match app.remove_at_targets(targets){
                Ok(_) => {},
                Err(e) => {println!("Failed to remove dice at provided targets with error {}" , e)}
            }
            }
        },
        Some(Commands::Reset) => {
            app.reset();
        },
        Some(Commands::Add {
            tray,
            dice_command
        }) => {
            match tray{
               Some(tray_string) => {
                    app.target_tray(tray_string);
                }
               None => {} 
            };
            if let Ok(raw_dice) = parse_dice_notation(dice_command){
                raw_dice.iter().for_each(|dice|{
                    app.add_dice_from_raw(dice.0, dice.1);
                });
            }
        },
        Some(Commands::Roll { 
            tray,
            targets 
        }) => {
            match tray{
                Some(tray_string) => {
                    app.target_tray(&tray_string);
                }
                None => {} 
            };
            match targets{
                Some(target_string) => {
                    if let Ok(targets) = parse_dice_targets(target_string){
                        match app.roll_at_targets(targets){
                            Ok(_) => {},
                            Err(e) => {println!("Failed to roll dice at provided targets with error {}" , e)}
                        }
                    }
                }
                None => {
                    app.roll_all();
                }
            }
        },
        None => {println!("No commands found!")}
    };

    app.show_tray();
    app.close();
}

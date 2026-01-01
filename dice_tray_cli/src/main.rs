mod cli_parser;
mod cli_dice_tray;
mod cli_dice_allocator;
mod logger;
mod app;

use cli_parser::parse_dice_notation;

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
    Add{
        #[arg(short, long)]
        tray: Option<String>,
        dice_command: String
    },
    Roll {
        #[arg(short, long)]
        tray: Option<String>,
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
                app.add_dice_from_raw(raw_dice.0, raw_dice.1);
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
                    todo!("Need to implement dice targeting.")
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

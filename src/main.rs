use clap::{Parser, Subcommand};
use log::LevelFilter;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Backy")]
#[command(author = "M.R")]
#[command(version = "1.0")]
#[command(about = "Backy - The backup tool")]
struct Args {
    #[arg(long, action=clap::ArgAction::SetTrue)]
    daemon: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { file: String },
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Add { file } => {
                log::info!("{}", file)
            }
        }
    }
}

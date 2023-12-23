pub mod config;
pub mod opts;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;

fn main() -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = opts::Opts::parse();

    if let Some(command) = args.command {
        match command {
            opts::Commands::Add(args) => opts::add::run(args)?,
        }
    }

    Ok(())
}

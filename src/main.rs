pub mod config;
pub mod opts;
pub mod logger;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    logger::init();

    let args = opts::Opts::parse();

    if let Some(command) = args.command {
        match command {
            opts::Commands::Add(args) => opts::add::run(args)?,
            opts::Commands::Remove(args) => opts::remove::run(args)?,
            opts::Commands::List => opts::list::run()?,
            opts::Commands::Push => opts::push::run()?,
        }
    }

    Ok(())
}

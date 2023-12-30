pub mod config;
pub mod logger;
pub mod opts;

use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;

fn main() -> Result<()> {
    logger::init();

    let args = opts::Opts::parse();
    match args.command {
        Some(command) => match command {
            opts::Commands::Add(args) => opts::add::run(args),
            opts::Commands::Remove(args) => opts::remove::run(args),
            opts::Commands::List => opts::list::run(),
            opts::Commands::Push => opts::push::run(),
            opts::Commands::Pull => opts::pull::run(),
            opts::Commands::Restore(args) => opts::restore::run(args),
        }?,
        None => opts::Opts::command().print_long_help()?,
    }
    Ok(())
}

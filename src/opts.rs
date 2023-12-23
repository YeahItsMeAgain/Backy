use clap::{Args, Parser, Subcommand};
use clio::ClioPath;

#[derive(Parser, Debug)]
#[command(name = "Backy")]
#[command(author = "M.R")]
#[command(version = "1.0")]
#[command(about = "Backy - The backup tool")]
pub struct Opts {
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub daemon: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(AddArgs),
    // TODO: Remove
    // TODO: Restore
    // TODO: Sync
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file().not_tty())]
    pub file: ClioPath,
}

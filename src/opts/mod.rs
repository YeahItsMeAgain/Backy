pub mod add;
pub mod list;
pub mod pull;
pub mod push;
pub mod remove;
pub mod vault;

use super::config;
use clap::{Args, Parser, Subcommand};
use clio::ClioPath;

#[derive(Parser, Debug)]
#[command(name = "Backy")]
#[command(author = "M.R")]
#[command(version = "1.0")]
#[command(about = "Backy - The backup tool")]
pub struct Opts {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(AddArgs),
    Remove(RemoveArgs),
    List,
    Push,
    Pull,
    // TODO: Restore
}

#[derive(Args, Debug)]
pub struct FileArgs {
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file().not_tty())]
    pub file: ClioPath,
}

type AddArgs = FileArgs;
type RemoveArgs = FileArgs;

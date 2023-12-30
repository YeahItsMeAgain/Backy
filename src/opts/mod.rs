pub mod add;
pub mod config;
pub mod list;
pub mod pull;
pub mod push;
pub mod remove;
pub mod restore;
pub mod vault;

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
    Restore(RestoreArgs),
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
pub struct FileArgs {
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file().not_tty())]
    pub file: ClioPath,
}

type AddArgs = FileArgs;
type RemoveArgs = FileArgs;

#[derive(Args, Debug)]
pub struct RestoreArgs {
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_file().not_tty())]
    pub file: Option<ClioPath>,

    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub all: Option<bool>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(long)]
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir().not_tty())]
    pub vault: Option<ClioPath>,

    #[arg(long)]
    pub repo: Option<String>,
}

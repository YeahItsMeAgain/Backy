pub mod config;
pub mod opts;
use anyhow::{Context, Result, bail};

use std::{fs, path::Path};

use clap::Parser;
use config::CONFIG;
use log::LevelFilter;

fn main() -> Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = opts::Opts::parse();

    if let Some(command) = args.command {
        match command {
            opts::Commands::Add(args) => {
                log::info!("Adding {}", args.file.display());

                fs::create_dir_all(&CONFIG.backup_folder).context(format!(
                    "Failed to create backup folder: {}",
                    CONFIG.backup_folder
                ))?;

                let full_path = fs::canonicalize(args.file.as_os_str()).unwrap();
                let backup_path = format!("{}{}", CONFIG.backup_folder, full_path.display());
                let backup_path = Path::new(&backup_path);
                if backup_path.exists() {
                    bail!("{} Already exists", backup_path.display());
                }

                fs::create_dir_all(backup_path.parent().unwrap()).context(format!(
                    "Failed to create hierarchy for: {}",
                    full_path.display()
                ))?;

                std::os::unix::fs::symlink(full_path.clone(), backup_path).context(
                    format!("Failed to create backup for {}", full_path.display()),
                )?;

                log::info!("Successfully Added {}", args.file.display());
            }
        }
    }

    Ok(())
}

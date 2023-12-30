use super::vault;
use super::AddArgs;
use crate::opts::config;

use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

pub fn run(args: AddArgs) -> Result<()> {
    log::info!("Adding {}", args.file.display());
    vault::init()?;

    let full_path = fs::canonicalize(args.file.as_os_str()).unwrap();
    let backup_path = format!("{}{}", config::get().vault, full_path.display());
    let backup_path = Path::new(&backup_path);
    if backup_path.exists() {
        bail!("{} Already exists", backup_path.display());
    }

    fs::create_dir_all(backup_path.parent().unwrap()).context(format!(
        "Failed to create hierarchy for: {}",
        full_path.display()
    ))?;

    std::fs::hard_link(full_path.clone(), backup_path).context(format!(
        "Failed to create backup for {}",
        full_path.display()
    ))?;

    vault::add_file(backup_path.as_os_str().to_str().unwrap().to_owned())?;

    log::info!("Successfully Added {}", args.file.display());
    Ok(())
}

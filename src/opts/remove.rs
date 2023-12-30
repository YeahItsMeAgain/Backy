use crate::opts::{config, vault};

use super::RemoveArgs;

use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

pub fn run(args: RemoveArgs) -> Result<()> {
    log::info!("Removing {}", args.file.display());

    let full_path = fs::canonicalize(args.file.as_os_str()).unwrap();
    let backup_path = format!("{}{}", config::get().vault, full_path.display());
    let backup_path = Path::new(&backup_path);
    if !backup_path.exists() {
        bail!("{} Doesn't exist", backup_path.display());
    }
    vault::rm(backup_path.as_os_str().to_str().unwrap().to_owned())
        .context(format!("Failed to remove {}", backup_path.display()))?;

    log::info!("Successfully Removed {}", args.file.display());
    Ok(())
}

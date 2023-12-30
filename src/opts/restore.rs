use super::vault;
use super::RestoreArgs;

use super::config::CONFIG;
use anyhow::Context;
use anyhow::{bail, Result};
use std::{fs, path::Path};

pub fn run(args: RestoreArgs) -> Result<()> {
    match args.file {
        Some(file) => restore_single_file(file.path()),
        None if args.all.is_some() => restore_all_files(),
        _ => Ok(()),
    }
}
fn restore_single_file(file: &Path) -> Result<()> {
    log::info!("Restoring {}", file.display());

    let full_path = fs::canonicalize(file).context("Failed to get full path")?;
    let backup_path = format!("{}{}", CONFIG.vault_path, full_path.display());
    let backup_path = Path::new(&backup_path);

    if !backup_path.exists() {
        bail!("{} Doesn't exist", backup_path.display());
    }

    fs::copy(backup_path, file).with_context(|| format!("Failed Restoring {}", file.display()))?;

    log::info!("Successfully Restored {}", file.display());
    Ok(())
}

fn restore_all_files() -> Result<()> {
    log::info!("Restoring Everything");

    match vault::list() {
        Ok(entries) => {
            for entry in entries {
                let backup_path = entry.clone();
                let original_path = entry.to_string_lossy();
                let original_path = original_path.strip_prefix(&CONFIG.vault_path).unwrap();

                fs::copy(backup_path, original_path)
                    .with_context(|| format!("Failed Restoring {}", original_path))?;

                log::info!("Restored {}", original_path);
            }
        }
        Err(err) => log::error!("{}", err),
    }

    log::info!("Successfully Restored Everything");
    Ok(())
}

use super::RemoveArgs;

use super::config::CONFIG;
use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

pub fn run(args: RemoveArgs) -> Result<()> {

    // TODO: git rm file + commit.
    log::info!("Removing {}", args.file.display());

    let full_path = fs::canonicalize(args.file.as_os_str()).unwrap();
    let backup_path = format!("{}{}", CONFIG.vault_path, full_path.display());
    let backup_path = Path::new(&backup_path);
    if !backup_path.exists() {
        bail!("{} Doesn't exist", backup_path.display());
    }

    fs::remove_file(backup_path).context(format!("Failed to remove {}", backup_path.display()))?;

    // Deleting all the empty folders.
    let vault_folder = fs::canonicalize(CONFIG.vault_path.clone()).unwrap();

    if let Some(mut folder_up) = backup_path.parent() {
        while folder_up.read_dir()?.next().is_none() {
            fs::remove_dir(folder_up)
                .context(format!("Failed to remove {}", folder_up.display()))?;

            // Stepping one folder up.
            match folder_up.parent() {
                Some(folder_up_up) => {
                    if folder_up_up == vault_folder {
                        break;
                    }
                    folder_up = folder_up_up;
                }
                None => {
                    break;
                }
            }
        }
    }

    log::info!("Successfully Removed {}", args.file.display());
    Ok(())
}

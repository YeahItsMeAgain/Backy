use super::{config::CONFIG, vault};
use anyhow::Result;

pub fn run() -> Result<()> {
    match vault::list() {
        Ok(entries) => {
            for entry in entries {
                log::info!(
                    "{}",
                    entry
                        .display()
                        .to_string()
                        .strip_prefix(&CONFIG.vault_path)
                        .unwrap()
                );
            }
        }
        Err(err) => log::error!("{}", err),
    }

    Ok(())
}

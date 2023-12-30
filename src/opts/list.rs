use super::vault;
use crate::opts::config;
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
                        .strip_prefix(&config::get().vault)
                        .unwrap()
                );
            }
        }
        Err(err) => log::error!("{}", err),
    }

    Ok(())
}

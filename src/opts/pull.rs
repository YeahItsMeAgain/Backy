use super::vault;
use anyhow::Result;

pub fn run() -> Result<()> {
    match vault::pull() {
        Ok(_) => log::info!("Successfully pulled changes"),
        Err(err) => log::error!("{}", err),
    }
    Ok(())
}

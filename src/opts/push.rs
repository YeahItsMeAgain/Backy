use super::vault;
use anyhow::Result;

pub fn run() -> Result<()> {
    match vault::push() {
        Ok(_) => log::info!("Successfully pushed changes"),
        Err(err) => log::error!("{}", err),
    }
    Ok(())
}

use anyhow::{Result, Context};
use preferences::{AppInfo, Preferences};
use std::fs;

use super::ConfigArgs;
const APP_INFO: AppInfo = AppInfo {
    name: "Backy",
    author: "M.R",
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub vault: String,
    pub repo: String,
}

pub fn get() -> Config {
    let mut config = Config {
        vault: "".to_owned(),
        repo: "".to_owned(),
    };

    if let Ok(existing_config) = Config::load(&APP_INFO, "config") {
        config.vault = existing_config.vault;
        config.repo = existing_config.repo;
    }
    config
}

pub fn run(args: ConfigArgs) -> Result<()> {
    let mut config = get();

    if let Some(repo) = args.repo {
        log::info!("Updating repo");
        config.repo = repo;
    } else if let Some(vault) = args.vault {
        log::info!("Updating vault folder");
        let vault_folder = fs::canonicalize(vault.path()).unwrap();
        config.vault = vault_folder.to_str().unwrap().to_owned();
    } else {
        log::info!("{:#?}", config);
    }

    config.save(&APP_INFO, "config").context("Error saving config")?;
    Ok(())
}

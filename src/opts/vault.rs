use std::{
    fs,
    process::{Command, Stdio},
};

use crate::config::CONFIG;
use anyhow::{Context, Result};
use std::path::Path;

pub fn init() -> Result<()> {
    if Path::join(Path::new(&CONFIG.vault_path), ".git").exists() {
        return Ok(());
    }

    fs::create_dir_all(&CONFIG.vault_path).context(format!(
        "Failed to create backup folder: {}",
        CONFIG.vault_path
    ))?;

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git init && git remote add origin {} && git lfs install",
            CONFIG.vault_repo
        ))
        .current_dir(&CONFIG.vault_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to init repo")?;

    if !output.stderr.is_empty() {
        log::error!("{}", String::from_utf8_lossy(&output.stderr))
    }
    Ok(())
}

use std::{
    fs,
    process::{Command, Stdio},
};

use crate::config::CONFIG;
use anyhow::{Context, Result};
use chrono;
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
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn push() -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git add .; git commit -m \"{}\"; git push origin master",
            chrono::offset::Local::now()
        ))
        .current_dir(&CONFIG.vault_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to push changes")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn pull() -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git pull origin master -q")
        .current_dir(&CONFIG.vault_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to pull changes")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

fn track_file(path: String) -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git lfs track \"*.{}\"; git add .gitattributes; git commit -m \"Updated .gitattributes\"",
            Path::new(&path).extension().unwrap().to_str().unwrap()
        ))
        .current_dir(&CONFIG.vault_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to track new file")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn add_file(path: String) -> Result<()> {
    track_file(path.clone())?;

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git add {} && git commit -m \"Added {} - {}\"",
            path,
            path,
            chrono::offset::Local::now()
        ))
        .current_dir(&CONFIG.vault_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to push changes")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

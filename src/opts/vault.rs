use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::opts::config;
use anyhow::{Context, Result};
use chrono;
use generator::{done, Generator, Gn};
use std::path::Path;
use walkdir::WalkDir;

pub fn init() -> Result<()> {
    if Path::join(Path::new(&config::get().vault), ".git").exists() {
        return Ok(());
    }

    fs::create_dir_all(config::get().vault).context(format!(
        "Failed to create backup folder: {}",
        config::get().vault
    ))?;

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git init && git remote add origin {} && git lfs install",
            config::get().repo
        ))
        .current_dir(&config::get().vault)
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
        .current_dir(&config::get().vault)
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
        .current_dir(&config::get().vault)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to pull changes")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn list() -> Result<Generator<'static, (), PathBuf>> {
    let vault_git_path = Path::join(Path::new(&config::get().vault), ".git");
    Ok(Gn::new_scoped(move |mut scope| {
        for entry in WalkDir::new(config::get().vault.clone())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().parent().unwrap() != Path::new(&config::get().vault)
                    && !e.path().starts_with(vault_git_path.clone())
            })
        {
            scope.yield_(entry.into_path());
        }
        done!();
    }))
}

fn track_file(path: String) -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git lfs track \"*.{}\"; git add .gitattributes; git commit -m \"Updated .gitattributes\"",
            Path::new(&path).extension().unwrap().to_str().unwrap()
        ))
        .current_dir(&config::get().vault)
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
            "git add {} && git commit -m \"Added {} - {}\" -q",
            path,
            path,
            chrono::offset::Local::now()
        ))
        .current_dir(&config::get().vault)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to push changes")?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

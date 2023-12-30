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

fn run_command(command: &str) -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(&config::get().vault)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context(format!("Failed to run command: {:?}", command))?;

    if !output.stderr.is_empty() {
        log::warn!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn init() -> Result<()> {
    if Path::join(Path::new(&config::get().vault), ".git").exists() {
        return Ok(());
    }

    fs::create_dir_all(config::get().vault).context(format!(
        "Failed to create backup folder: {}",
        config::get().vault
    ))?;

    run_command("git init && git remote add origin {} && git lfs install")
        .context("Failed to init repo")?;
    Ok(())
}

pub fn push() -> Result<()> {
    run_command(
        format!(
            "git add .; git commit -m \"{}\" -q; git push origin master -q",
            chrono::offset::Local::now()
        )
        .as_str(),
    )
    .context("Failed to push changes")?;
    Ok(())
}

pub fn pull() -> Result<()> {
    run_command("git pull origin master -q").context("Failed to pull changes")?;
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
    run_command(
        format!(
            "git lfs track \"*.{}\"; git add .gitattributes; git commit -m \"Updated .gitattributes\"",
            Path::new(&path).extension().unwrap().to_str().unwrap()
        )
        .as_str(),
    )
    .context("Failed to track new file")?;
    Ok(())
}

pub fn add(path: String) -> Result<()> {
    track_file(path.clone())?;

    run_command(
        format!(
            "git add {} && git commit -m \"Added {} - {}\" -q",
            path,
            path,
            chrono::offset::Local::now()
        )
        .as_str(),
    )
    .context("Failed to push changes")?;
    Ok(())
}

pub fn rm(path: String) -> Result<()> {
    run_command(
        format!(
            "git rm {} && git commit -m \"Removed {} - {}\" -q",
            path,
            path,
            chrono::offset::Local::now()
        )
        .as_str(),
    )
    .context(format!("Failed to remove {}", path))?;
    Ok(())
}

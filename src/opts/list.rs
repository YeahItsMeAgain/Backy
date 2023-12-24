use super::config::CONFIG;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    let vault_git_path = Path::join(Path::new(&CONFIG.vault_path), ".git");

    for entry in WalkDir::new(CONFIG.vault_path.clone())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.path().starts_with(vault_git_path.clone()) && e.file_type().is_file())
    {
        let entry_link = entry.path();
        log::info!(
            "{}",
            entry_link
                .display()
                .to_string()
                .strip_prefix(&CONFIG.vault_path)
                .unwrap()
        );
    }
    Ok(())
}

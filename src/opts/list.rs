use super::config::CONFIG;
use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

pub fn run() -> Result<()> {
    for entry in WalkDir::new(CONFIG.backup_folder.clone())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_symlink())
    {
        let entry_link = entry.path();
        let entry_path = match fs::read_link(entry_link) {
            Ok(link_path) => link_path.into_os_string().into_string().unwrap(),
            Err(_) => "Broken symlink".to_owned(),
        };

        log::info!("{} -> {}", entry_link.display(), entry_path);
    }
    Ok(())
}

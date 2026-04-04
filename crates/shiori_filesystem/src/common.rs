// TODO: Add some sort of file parts extension for file stuff
// tahts gets file name, stem and type.

use std::fs as sync_fs;
use std::path::{Path, PathBuf};

use tokio::fs;

pub async fn move_file(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    fs::rename(source, dest).await
}

pub async fn delete_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    fs::remove_file(path).await
}

pub struct Directory {
    pub name: String,
    pub path: String,
    pub has_children: bool,
}

pub fn list_directories(path: impl AsRef<Path>) -> std::io::Result<Vec<Directory>> {
    let mut directories = Vec::new();
    let entries = sync_fs::read_dir(&path)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            let dir_path = entry.path();

            let has_children = match sync_fs::read_dir(&dir_path) {
                Ok(mut sub_entries) => sub_entries.any(|e| {
                    e.ok().map_or(false, |d| {
                        d.file_type().map(|t| t.is_dir()).unwrap_or(false)
                    })
                }),
                Err(_) => false,
            };

            directories.push(Directory {
                name: entry.file_name().to_string_lossy().to_string(),
                path: dir_path.to_string_lossy().to_string(),
                has_children,
            });
        }
    }

    Ok(directories)
}

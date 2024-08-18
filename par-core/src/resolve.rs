use anyhow::anyhow;
use std::path;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Locations {
    pub files: Vec<String>,
    pub dirs: Vec<String>,
}

pub fn get_existing_locations(base: &Path) -> anyhow::Result<Locations> {
    let walk_dir = WalkDir::new(base);
    let mut locations = Locations {
        files: Vec::with_capacity(16),
        dirs: Vec::with_capacity(16),
    };
    for entry in walk_dir {
        let entry = entry?;
        let mut path = pathdiff::diff_paths(entry.path(), base)
            .ok_or(anyhow!("Error resolving relative path"))?
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow!("Encountered invalid UTF-8"))?;
        if path.starts_with('.') {
            continue;
        }
        if cfg!(windows) {
            path = path.replace(path::MAIN_SEPARATOR, "/")
        }
        if entry.path().is_dir() {
            if !path.is_empty() {
                let dir_path = format!("{path}/");
                if !locations.dirs.contains(&dir_path) {
                    locations.dirs.push(dir_path)
                }
            }
        } else {
            locations.files.push(path)
        }
    }
    Ok(locations)
}

pub fn resolve_existing_location(
    base: &Path,
    key: &str,
    can_be_dir: bool,
) -> anyhow::Result<PathBuf> {
    let concat = base.join(key);
    if can_be_dir {
        return if concat.exists() {
            Ok(concat)
        } else {
            Err(anyhow!("Key {key} does not exist"))
        };
    }
    if concat.is_file() {
        return Ok(concat);
    }
    if concat.is_dir() {
        let pass = concat.join("pass");
        return if pass.is_file() {
            Ok(pass)
        } else {
            Err(anyhow!("{key} is a directory"))
        };
    }
    let existing = get_existing_locations(base)?;
    let candidates: Vec<_> = existing
        .files
        .iter()
        .filter(|&s| s.starts_with(key))
        .collect();
    if candidates.len() == 1 {
        return Ok(base.join(candidates[0]));
    }
    Err(anyhow!("Key {key} does not exist"))
}

pub fn resolve_new_location(base: &Path, key: &str) -> anyhow::Result<PathBuf> {
    let file = base.join(key);
    if file.exists() {
        Err(anyhow!("Key {key} already exists"))
    } else {
        Ok(file)
    }
}

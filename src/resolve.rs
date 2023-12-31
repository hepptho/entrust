use crate::error::ParResult;
use anyhow::anyhow;
use std::path::{Path, PathBuf};
use std::{io, path};
use walkdir::WalkDir;

pub fn get_existing_locations(base: &Path) -> io::Result<Vec<String>> {
    let walk_dir = WalkDir::new(base);
    let mut ret = Vec::new();
    for entry in walk_dir {
        let entry = entry?;
        if !entry.path().is_file() {
            continue;
        }
        let path = pathdiff::diff_paths(entry.path(), base)
            .ok_or(io::Error::other("Error resolving relative path"))?
            .into_os_string()
            .into_string()
            .map_err(|_| io::Error::other("Encountered invalid UTF-8"))?;
        if path.starts_with('.') {
            continue;
        }
        if cfg!(windows) {
            ret.push(path.replace(path::MAIN_SEPARATOR, "/"));
        } else {
            ret.push(path);
        }
    }
    Ok(ret)
}

pub fn resolve_existing(base: &Path, key: &str, can_be_dir: bool) -> ParResult<PathBuf> {
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
        return Err(anyhow!("{key} is a directory"));
    };
    if concat.is_dir() {
        let pass = concat.join("pass");
        if pass.is_file() {
            return Ok(pass);
        }
    }
    let existing = get_existing_locations(base)?;
    let candidates: Vec<_> = existing.iter().filter(|&s| s.starts_with(key)).collect();
    if candidates.len() == 1 {
        return Ok(base.join(candidates[0]));
    }
    Err(anyhow!("Key {key} does not exist"))
}

pub fn resolve_new(base: &Path, key: &str) -> ParResult<PathBuf> {
    let file = base.join(key);
    if file.exists() {
        Err(anyhow!("Key {key} already exists"))
    } else {
        Ok(file)
    }
}

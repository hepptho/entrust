use crate::key::Key;
use anyhow::anyhow;
use clap::Args;
use entrust_core::git;
use entrust_core::resolve_existing_location;
use std::fs;
use std::path::PathBuf;

pub(super) const ABOUT: &str = "Delete a password";

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The key to delete
    #[arg()]
    key: Option<String>,
    /// Enable deleting directories
    #[arg(short, long)]
    recurse: bool,
}

pub fn run(store: PathBuf, args: RemoveArgs) -> anyhow::Result<()> {
    let key = &args.key.unwrap_or_select_existing(&store)?;
    let location = resolve_existing_location(&store, key, true)?;
    let is_dir = location.is_dir();
    if is_dir && !args.recurse {
        return Err(anyhow!(
            "{} is a directory; specify --recurse to delete",
            key
        ));
    }
    if is_dir {
        fs::remove_dir_all(&location)?;
    } else {
        fs::remove_file(&location)?;
    };
    git::remove(&store, key)?;
    if let Some(parent) = location.parent() {
        if parent.exists() && parent.read_dir()?.next().is_none() {
            fs::remove_dir(parent)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use tempfile::TempDir;

    fn setup() -> io::Result<TempDir> {
        let dir = tempfile::tempdir()?;
        fs::create_dir(dir.path().join("subdir"))?;
        File::create(dir.path().join("subdir/file1"))?;
        File::create(dir.path().join("subdir/file2"))?;
        Ok(dir)
    }

    #[test]
    fn test_remove_non_recursive() {
        let dir = setup().unwrap();

        let result_file = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir/file1".to_string()),
                recurse: false,
            },
        );
        assert!(result_file.is_ok());

        let result_dir = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir".to_string()),
                recurse: false,
            },
        );
        assert!(result_dir.is_err_and(|e| e.to_string().contains("is a directory")));
    }

    #[test]
    fn test_remove_recursive() {
        let dir = setup().unwrap();

        let result_file = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir/file1".to_string()),
                recurse: true,
            },
        );
        assert!(result_file.is_ok());

        let result_dir = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir".to_string()),
                recurse: true,
            },
        );
        assert!(result_dir.is_ok());
    }

    #[test]
    fn test_cleanup_empty_dir() {
        let dir = setup().unwrap();

        let result_file = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir/file1".to_string()),
                recurse: false,
            },
        );
        assert!(result_file.is_ok());
        let result_file = run(
            dir.path().to_path_buf(),
            RemoveArgs {
                key: Some("subdir/file2".to_string()),
                recurse: false,
            },
        );
        assert!(result_file.is_ok());

        assert!(!dir.path().join("subdir").exists());
    }
}

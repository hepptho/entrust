use crate::key::Key;
use anyhow::anyhow;
use clap::Args;
use par_core::git;
use par_core::resolve_existing_location;
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
        if parent.exists() && parent.read_dir().iter().next().is_none() {
            fs::remove_dir(parent)?;
        }
    }
    Ok(())
}

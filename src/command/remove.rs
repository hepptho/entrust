use crate::error::ParResult;
use crate::git;
use crate::resolve::resolve_existing;
use anyhow::anyhow;
use clap::Args;
use std::fs;
use std::path::PathBuf;

pub(super) const ABOUT: &str = "Delete a password";

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The key to delete
    #[arg()]
    key: String,
    /// Enable deleting directories
    #[arg(short, long)]
    recurse: bool,
}

pub fn run(home: PathBuf, args: RemoveArgs) -> ParResult<()> {
    let location = resolve_existing(&home, &args.key, true)?;
    let is_dir = location.is_dir();
    if is_dir && !args.recurse {
        return Err(anyhow!(
            "{} is a directory; specify --recurse to delete",
            args.key
        ));
    }
    if is_dir {
        fs::remove_dir_all(&location)?;
    } else {
        fs::remove_file(&location)?;
    };
    git::remove(&home, &args.key)?;
    if let Some(parent) = location.parent() {
        if parent.exists() && parent.read_dir().iter().next().is_none() {
            fs::remove_dir(parent)?;
        }
    }
    Ok(())
}

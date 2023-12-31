use crate::error::ParResult;
use crate::git;
use crate::resolve::{resolve_existing, resolve_new};
use clap::Args;
use std::fs;
use std::path::PathBuf;

pub(super) const ABOUT: &str = "Move a password to another location in the store";

#[derive(Args, Debug)]
pub struct MoveArgs {
    #[arg()]
    from: String,
    #[arg()]
    to: String,
}

pub fn run(home: PathBuf, args: MoveArgs) -> ParResult<()> {
    let from_location = resolve_existing(&home, &args.from, true)?;
    let to_location = resolve_new(&home, &args.to)?;
    if let Some(dir) = to_location.parent() {
        fs::create_dir_all(dir)?;
    }
    let git_moved = git::r#move(&home, &args.from, &args.to)?;
    if !git_moved {
        fs::rename(from_location, to_location)?;
    }
    Ok(())
}

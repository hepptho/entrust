use crate::key::Key;
use crate::theme::chevron_prompt;
use clap::Args;
use entrust_core::git;
use entrust_core::{resolve_existing_location, resolve_new_location};
use std::fs;
use std::path::PathBuf;

pub(super) const ABOUT: &str = "Move a password to another location in the store";

#[derive(Args, Debug)]
pub struct MoveArgs {
    #[arg()]
    from: Option<String>,
    #[arg()]
    to: Option<String>,
}

pub fn run(store: PathBuf, args: MoveArgs) -> anyhow::Result<()> {
    let from = &args.from.unwrap_or_select_existing(&store)?;
    let to = &args
        .to
        .unwrap_or_read_new(chevron_prompt!("New key"), &store)?;
    let from_location = resolve_existing_location(&store, from, true)?;
    let to_location = resolve_new_location(&store, to)?;
    if let Some(dir) = to_location.parent() {
        fs::create_dir_all(dir)?;
    }
    let git_moved = git::r#move(&store, from, to)?;
    if !git_moved {
        fs::rename(from_location, to_location)?;
    }
    Ok(())
}

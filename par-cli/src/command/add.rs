use clap::Args;
use color_print::cstr;
use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::command::BackendValueEnum;
use crate::dialog;
use crate::key::Key;
use crate::theme::chevron_prompt;
use par_core;
use par_core::{git, Backend};

pub(super) const ABOUT: &str = "Add a new password";

pub(super) const LONG_ABOUT: &str = cstr!(
    "

  Add a new password

  Reads from <bold,#ffb86c>stdin</> or offers an interactive prompt if stdin is empty"
);

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The key under which to store the encrypted file
    key: Option<String>,
    /// Choose gpg or age for encryption
    #[arg(short, long, value_enum, default_value_t = BackendValueEnum::Age)]
    pub backend: BackendValueEnum,
    /// Do not add the new file to git
    #[arg(long = "no-git")]
    no_git: bool,
}

pub fn run(store: PathBuf, args: AddArgs) -> anyhow::Result<()> {
    let key = &args
        .key
        .unwrap_or_read_new(chevron_prompt!("Key"), &store)?;
    encrypt(&store, key, args.backend.into())?;
    if !args.no_git {
        git::add(&store, key)?
    }
    Ok(())
}

fn encrypt(store: &Path, key: &str, backend: Backend) -> anyhow::Result<()> {
    let location = par_core::resolve_new_location(store, key)?;
    if let Some(parent) = location.parent() {
        fs::create_dir_all(parent)?;
    }
    let input = if io::stdin().is_terminal() {
        dialog::read_password_interactive("")?
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    backend.encrypt(input.as_bytes(), store, &location)?;
    Ok(())
}

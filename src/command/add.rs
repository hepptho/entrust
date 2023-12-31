use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Args;
use dialoguer::Password;
use log::{debug, info};

use crate::backend::encrypt_with_backend;
use crate::command::BackendOption;
use crate::error::ParResult;
use crate::theme::DIALOGUER_THEME;
use crate::{git, resolve};

pub(super) const ABOUT: &str = "Add a new password";

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The key under which to store the encrypted file
    key: String,
    /// Choose gpg or age for en-/decryption
    #[arg(short, long, value_enum, default_value_t = BackendOption::Age)]
    backend: BackendOption,
    /// Do not add the new file to git
    #[arg(long = "no-git")]
    no_git: bool,
}

pub fn run(home: PathBuf, args: AddArgs) -> ParResult<()> {
    debug!("add run");
    encrypt(&home, &args)?;
    if !args.no_git {
        git::add(&home, &args.key)?
    }
    info!("Added {}", args.key);
    Ok(())
}

fn encrypt(home: &Path, args: &AddArgs) -> ParResult<()> {
    let location = resolve::resolve_new(home, &args.key)?;
    debug!("Location: {:?}", location);
    if let Some(parent) = location.parent() {
        fs::create_dir_all(parent)?;
    }
    let input = if io::stdin().is_terminal() {
        read_password_interactive()?
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    encrypt_with_backend(&args.backend, input.as_bytes(), home, &location)?;
    Ok(())
}

pub(crate) fn read_password_interactive() -> ParResult<String> {
    let pass = Password::with_theme(&*DIALOGUER_THEME)
        .with_prompt("Enter new Password")
        .with_confirmation("Confirm password", "The entered passwords do not match.")
        .report(false)
        .interact()?;
    Ok(pass)
}

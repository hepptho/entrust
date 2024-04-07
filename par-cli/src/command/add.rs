use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Args;
use color_print::cstr;
use inquire::validator::Validation;
use inquire::PasswordDisplayMode;
use log::{debug, info};

use par_core;
use par_core::{git, Backend};

use crate::command::BackendValueEnum;
use crate::theme::INQUIRE_RENDER_CONFIG;

pub(super) const ABOUT: &str = "Add a new password";

pub(super) const LONG_ABOUT: &str = cstr!(
    "

  Add a new password

  Reads from <bold,#ffb86c>stdin</> or offers an interactive prompt if stdin is empty"
);

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The key under which to store the encrypted file
    key: String,
    /// Choose gpg or age for encryption
    #[arg(short, long, value_enum, default_value_t = BackendValueEnum::Age)]
    pub backend: BackendValueEnum,
    /// Do not add the new file to git
    #[arg(long = "no-git")]
    no_git: bool,
}

pub fn run(store: PathBuf, args: AddArgs) -> anyhow::Result<()> {
    debug!("add run");
    encrypt(&store, &args)?;
    if !args.no_git {
        git::add(&store, &args.key)?
    }
    info!("Added {}", args.key);
    Ok(())
}

fn encrypt(store: &Path, args: &AddArgs) -> anyhow::Result<()> {
    let location = par_core::resolve_new_location(store, &args.key)?;
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
    Backend::from(args.backend).encrypt(input.as_bytes(), store, &location)?;
    Ok(())
}

pub(crate) fn read_password_interactive() -> anyhow::Result<String> {
    let pass = inquire::Password::new("Enter new Password ❯")
        .with_render_config(*INQUIRE_RENDER_CONFIG)
        .with_custom_confirmation_message("Confirm password ❯")
        .with_custom_confirmation_error_message("The entered passwords do not match.")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid(
                    "The password must not be empty.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;
    Ok(pass)
}

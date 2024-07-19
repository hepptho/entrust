use clap::Args;
use color_print::cstr;
use log::{debug, info};
use std::io::{IsTerminal, Read};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::command::BackendValueEnum;
use crate::theme::DIALOG_THEME;
use par_core;
use par_core::{git, Backend};
use par_dialog::dialog::Dialog;
use par_dialog::input::confirmation::{Confirmation, ConfirmationMessageType};
use par_dialog::input::prompt::Prompt;
use par_dialog::input::validator::Validator;
use par_dialog::input::InputDialog;

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
        read_password_interactive("")?
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    Backend::from(args.backend).encrypt(input.as_bytes(), store, &location)?;
    Ok(())
}

pub(crate) fn read_password_interactive(initial: &str) -> anyhow::Result<String> {
    let pass = InputDialog::default()
        .with_content(initial)
        .with_prompt(Prompt::inline("Enter new password ❯ "))
        .with_confirmation(Confirmation::new(
            "Confirm password   ❯ ",
            "The entered passwords do not match ❯ ",
            ConfirmationMessageType::Inline,
        ))
        .with_validator(Validator::not_empty("The password must not be empty."))
        .with_hidden(initial.is_empty())
        .with_theme(DIALOG_THEME.deref())
        .run()?;
    Ok(pass)
}

use std::fs;
use std::path::PathBuf;

use clap::Args;
use color_print::cstr;
use crossterm::style::Stylize;

use crate::backend::Backend;
use crate::command::add::read_password_interactive;
use crate::git;
use crate::resolve::resolve_existing;

pub(super) const ABOUT: &str = "Change an existing password";

pub(super) const LONG_ABOUT: &str = cstr!(
    "

  Change an existing password

  Displays the old password and offers an interactive prompt.
  To overwrite the password without interaction, <bold,#ffb86c>remove</> and <bold,#ffb86c>add</>."
);

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The key of the password to edit
    key: String,
    /// Choose gpg or age for re-encryption
    #[arg(short, long, value_enum, default_value_t = Backend::Age)]
    pub backend: Backend,
}

pub fn run(store: PathBuf, args: EditArgs) -> anyhow::Result<()> {
    let location = resolve_existing(&store, &args.key, false)?;

    let mut bak = location.clone();
    bak.as_mut_os_string().push(".bak");
    fs::rename(&location, &bak)?;

    let old = Backend::decrypt(&bak)?;
    eprintln!("{} {}", "Old password:".blue(), old.bold());
    let new = read_password_interactive()?;
    args.backend.encrypt(new.as_bytes(), &store, &location)?;

    git::edit(&store, &args.key)?;
    fs::remove_file(bak)?;
    Ok(())
}

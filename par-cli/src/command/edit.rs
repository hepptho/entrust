use std::borrow::Cow;
use std::fs;
use std::io::{stdin, IsTerminal};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use clap::Args;
use color_print::cstr;

use crate::command::add::read_password_interactive;
use crate::command::BackendValueEnum;
use par_core::{git, resolve_existing_location, Backend};

pub(super) const ABOUT: &str = "Change an existing password";

pub(super) const LONG_ABOUT: &str = cstr!(
    "

  Change an existing password

  Displays the old password and offers an interactive prompt if <bold,#ffb86c>stdin</> is empty, \
  otherwise reads a line from stdin."
);

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The key of the password to edit
    key: String,
    /// Edit the password in cleartext
    ///
    /// Only effective when stdin is empty
    #[arg(short, long)]
    cleartext: bool,
    /// Choose gpg or age for re-encryption
    #[arg(short, long, value_enum, default_value_t = BackendValueEnum::Age)]
    pub backend: BackendValueEnum,
}

pub fn run(store: PathBuf, args: EditArgs) -> anyhow::Result<()> {
    let location = resolve_existing_location(&store, &args.key, false)?;

    let mut bak = location.clone();
    bak.as_mut_os_string().push(".bak");
    fs::rename(&location, &bak)?;

    let edited = if stdin().is_terminal() {
        edit_interactive(&args, &bak)
    } else {
        edit_non_interactive()
    };

    let encryption_result =
        edited.and_then(|e| Backend::from(args.backend).encrypt(e.as_bytes(), &store, &location));
    if encryption_result.is_ok() {
        git::edit(&store, &args.key)?;
        fs::remove_file(bak)?;
    } else {
        fs::rename(&bak, &location)?;
    }
    encryption_result
}

fn edit_interactive(args: &EditArgs, bak: &Path) -> anyhow::Result<String> {
    let initial = if args.cleartext {
        Cow::Owned(Backend::decrypt(bak)?)
    } else {
        Cow::Borrowed("")
    };
    read_password_interactive(initial.deref())
}

fn edit_non_interactive() -> anyhow::Result<String> {
    let mut buf = String::with_capacity(16);
    stdin().read_line(&mut buf)?;
    Ok(buf)
}

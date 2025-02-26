use std::borrow::Cow;
use std::fs;
use std::io::{IsTerminal, Read, stdin};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use clap::Args;
use color_print::cstr;

use crate::command::BackendValueEnum;
use crate::dialog::read_password_interactive;
use crate::key::Key;
use entrust_core::{Backend, git, resolve_existing_location};

pub(super) const ABOUT: &str = "Change an existing password";

pub(super) const LONG_ABOUT: &str = cstr!(
    "

  Change an existing password

  Displays the old password and offers an interactive prompt if <bold,#ffb86c>stdin</> is empty, \
  otherwise reads from stdin."
);

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The key of the password to edit
    key: Option<String>,
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
    let key = &args.key.unwrap_or_select_existing(&store)?;
    let location = resolve_existing_location(&store, key, false)?;

    let edited = if stdin().is_terminal() {
        edit_interactive(args.cleartext, &location)
    } else {
        edit_non_interactive()
    }?;

    let mut bak = location.clone();
    bak.as_mut_os_string().push(".bak");
    fs::rename(&location, &bak)?;

    let encryption_result =
        Backend::from(args.backend).encrypt(edited.as_bytes(), &store, &location);
    if encryption_result.is_ok() {
        git::edit(&store, key)?;
        fs::remove_file(bak)?;
    } else {
        fs::rename(&bak, &location)?;
    }
    encryption_result
}

fn edit_interactive(cleartext: bool, bak: &Path) -> anyhow::Result<String> {
    let initial = if cleartext {
        Cow::Owned(Backend::decrypt(bak)?)
    } else {
        Cow::Borrowed("")
    };
    read_password_interactive(initial.deref())
}

fn edit_non_interactive() -> anyhow::Result<String> {
    let mut buf = String::with_capacity(16);
    stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

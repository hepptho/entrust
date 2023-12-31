use crate::backend::encrypt_with_backend;
use crate::command::add::read_password_interactive;
use crate::command::BackendOption;
use crate::error::ParResult;
use crate::git;
use crate::resolve::resolve_existing;
use clap::Args;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::{fs, io};

pub(super) const ABOUT: &str = "Change an existing password";

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The key of the password to edit
    key: String,
    /// Choose gpg or age for en-/decryption
    #[arg(short, long, value_enum, default_value_t = BackendOption::Age)]
    backend: BackendOption,
}

pub fn run(home: PathBuf, args: EditArgs) -> ParResult<()> {
    let location = resolve_existing(&home, &args.key, false)?;

    let mut bak = location.clone();
    bak.as_mut_os_string().push(".bak");
    fs::rename(&location, &bak)?;

    let result = if io::stdin().is_terminal() {
        encrypt_with_backend(
            &args.backend,
            read_password_interactive()?.as_bytes(),
            &home,
            &location,
        )
    } else {
        encrypt_with_backend(&args.backend, io::stdin(), &home, &location)
    };
    git::edit(&home, &args.key)?;
    fs::remove_file(bak)?;
    result
}

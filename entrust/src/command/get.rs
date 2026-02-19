use crate::command::clip;
use crate::key::Key;
use clap::Args;
use color_print::cstr;
use entrust_core::{Backend, resolve_existing_location};
use std::io;
use std::io::IsTerminal;
use std::path::PathBuf;

pub(super) const ABOUT: &str = "Decrypt a password";

pub(super) const LONG_ABOUT: &str = cstr!("

  Decrypt a password

  The age identity for age-encrypted files can be provided in <bold,#ffb86c>AGE_IDENTITY</> or piped into <bold,#ffb86c>stdin</>");

#[derive(Args, Debug)]
pub struct GetArgs {
    /// The key of the password to decrypt
    key: Option<String>,
    /// Copy the password to the clipboard
    #[arg(short, long)]
    pub(super) clipboard: bool,
    /// Clear the clipboard after the given number of seconds.
    /// Pass 0 to disable clearing
    #[arg(short = 'd', long, default_value = "10")]
    pub(super) clear_clipboard_delay: u64,
}

pub fn run(store: PathBuf, args: GetArgs) -> anyhow::Result<()> {
    let location = &args
        .key
        .unwrap_or_select_existing(&store)
        .and_then(|key| resolve_existing_location(&store, &key, false))?;
    let decrypted = Backend::decrypt(location)?;

    if args.clipboard && args.clear_clipboard_delay > 0 {
        clip::clear_in_new_process(decrypted.as_str(), args.clear_clipboard_delay)?;
        clip::copy(decrypted.into())?;
    } else {
        print!("{decrypted}");
        if io::stdout().is_terminal() {
            println!()
        }
    };
    Ok(())
}

use crate::command::clear_clipboard;
use crate::key::Key;
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use copypasta::{ClipboardContext, ClipboardProvider};
use entrust_core::{resolve_existing_location, Backend};
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
    #[arg(
        short,
        long,
        default_missing_value = "10",
        num_args = 0..=1,
        require_equals = true,
        value_name = "CLEAR AFTER SECONDS",
    )]
    clipboard: Option<u32>,
}

pub fn run(store: PathBuf, args: GetArgs) -> anyhow::Result<()> {
    let location = &args
        .key
        .unwrap_or_select_existing(&store)
        .and_then(|key| resolve_existing_location(&store, &key, false))?;
    let decrypted = Backend::decrypt(location)?;

    if let Some(clear_after_seconds) = args.clipboard {
        clear_clipboard::clear_in_new_process(decrypted.as_str(), clear_after_seconds)?;
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.set_contents(decrypted))
            .map_err(|_| anyhow!("Could not access Clipboard"))?;
    } else {
        print!("{decrypted}");
        if io::stdout().is_terminal() {
            println!()
        }
    };
    Ok(())
}

use crate::command::clear_clipboard;
use crate::key::Key;
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use copypasta::{ClipboardContext, ClipboardProvider};
use par_core::{resolve_existing_location, Backend};
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
    clipboard: bool,
    /// Delay in seconds after which the clipboard should be cleared
    /// (only effective with --clipboard)
    #[arg(
        short = 'd',
        long,
        default_value_t = 10,
        value_name = "SECONDS",
        requires = "clipboard"
    )]
    clear_clipboard_delay: u32,
}

pub fn run(store: PathBuf, args: GetArgs) -> anyhow::Result<()> {
    let location = &args
        .key
        .unwrap_or_select_existing(&store)
        .and_then(|key| resolve_existing_location(&store, &key, false))?;
    let decrypted = Backend::decrypt(location)?;
    if args.clipboard {
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.set_contents(decrypted))
            .map_err(|_| anyhow!("Could not access Clipboard"))?;
        clear_clipboard::clear_in_new_process(args.clear_clipboard_delay)?;
    } else {
        print!("{decrypted}");
        if io::stdout().is_terminal() {
            println!()
        }
    };
    Ok(())
}

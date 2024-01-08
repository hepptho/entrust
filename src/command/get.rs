use crate::backend::Backend;
use crate::command::clear_clipboard;
use crate::error::ParResult;
use crate::resolve::{get_existing_locations, resolve_existing};
use crate::theme::INQUIRE_RENDER_CONFIG;
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use copypasta::{ClipboardContext, ClipboardProvider};
use inquire::Select;
use std::io;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

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
    /// (only effective with clipboard output)
    #[arg(
        short = 'd',
        long,
        default_value_t = 10,
        value_name = "INT",
        requires = "clipboard"
    )]
    clear_clipboard_delay: u32,
}

pub fn run(store: PathBuf, args: GetArgs) -> ParResult<()> {
    let location = get_location(&store, &args.key)?;
    let decrypted = Backend::decrypt(&location)?;
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

fn get_location(store: &Path, key: &Option<String>) -> ParResult<PathBuf> {
    match key {
        Some(k) => resolve_existing(store, k, false),
        None => select_key(store).map(|k| store.join(k)),
    }
}

fn select_key(store: &Path) -> ParResult<String> {
    let vec = get_existing_locations(store)?;
    let selected = Select::new("Select key", vec)
        .with_render_config(*INQUIRE_RENDER_CONFIG)
        .with_page_size(15)
        .prompt()?;
    Ok(selected)
}

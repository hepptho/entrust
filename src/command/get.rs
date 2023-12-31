use crate::backend::decrypt_file;
use crate::command::clear_clipboard;
use crate::error::ParResult;
use crate::resolve::{get_existing_locations, resolve_existing};
use crate::theme::DIALOGUER_THEME;
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use copypasta::{ClipboardContext, ClipboardProvider};
use dialoguer::FuzzySelect;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub(super) const ABOUT: &str = "Decrypt a password";

pub(super) const LONG_ABOUT: &str = cstr!("

  Decrypt a password

  The age identity for age-encrypted files can be provided in <bold,#ffb86c>AGE_IDENTITY</> or piped into <bold,#ffb86c>stdin</>");

#[derive(Args, Debug)]
pub struct GetArgs {
    #[arg(default_value = "")]
    key: String,
    #[command(flatten)]
    output: Option<OutputArgs>,
    /// Delay in seconds after which the clipboard should be cleared
    /// (only effective with clipboard output)
    #[arg(short = 'd', long, default_value_t = 10, value_name = "INT")]
    clear_clipboard_delay: u32,
}

impl GetArgs {
    fn output_type(&self) -> OutputType {
        match &self.output {
            Some(OutputArgs {
                clipboard: true,
                file: _,
            }) => OutputType::Clipboard,
            Some(OutputArgs {
                clipboard: false,
                file: Some(file),
            }) => OutputType::File(file),
            _ => OutputType::Stdout,
        }
    }
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct OutputArgs {
    /// Copy the password to the clipboard,
    #[arg(short, long, help_heading = "Output")]
    clipboard: bool,
    /// Write the password to a file
    #[arg(short, long, help_heading = "Output")]
    file: Option<PathBuf>,
}

enum OutputType<'a> {
    Stdout,
    Clipboard,
    File(&'a Path),
}

pub fn run(home: PathBuf, args: GetArgs) -> ParResult<()> {
    let location = get_location(&home, &args.key)?;
    let decrypted = decrypt_file(&location)?;
    match args.output_type() {
        OutputType::Stdout => {
            print!("{decrypted}");
            if io::stdout().is_terminal() {
                println!()
            }
        }
        OutputType::Clipboard => {
            ClipboardContext::new()
                .and_then(|mut ctx| ctx.set_contents(decrypted))
                .map_err(|_| anyhow!("Could not access Clipboard"))?;
            clear_clipboard::clear_in_new_process(args.clear_clipboard_delay)?;
        }
        OutputType::File(file) => fs::write(file, decrypted)?,
    }
    Ok(())
}

fn get_location(home: &Path, key: &str) -> ParResult<PathBuf> {
    match resolve_existing(home, key, false) {
        Ok(path) => Ok(path),
        Err(_) => select_location(home, key),
    }
}

fn select_location(home: &Path, key: &str) -> ParResult<PathBuf> {
    let vec = get_existing_locations(home)?;
    let sel = FuzzySelect::with_theme(&*DIALOGUER_THEME)
        .with_prompt("Select key")
        .with_initial_text(key)
        .items(&vec)
        .report(false)
        .interact()?;
    let selected = &vec[sel];
    resolve_existing(home, selected, false)
}

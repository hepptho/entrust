use anyhow::anyhow;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::{fs, io};

use copypasta::{ClipboardContext, ClipboardProvider};
use par_core::{generate_passphrase, generate_password, Backend};

use crate::command::clear_clipboard;
use crate::command::generate::animation::animate;
use crate::command::generate::parse::{GenerateArgs, Output, Type};
use par_core::git;

mod animation;
pub mod parse;

pub(crate) const ABOUT: &str = "Generate a random password";

pub fn run(store: PathBuf, args: GenerateArgs) -> anyhow::Result<()> {
    let pass = match &args.r#type {
        Type::Phrase => generate_passphrase(args.length(), &args.separator),
        Type::Word => generate_password(args.length()),
    };
    output(&store, args, pass)?;
    Ok(())
}

fn output(store: &Path, args: GenerateArgs, pass: String) -> anyhow::Result<()> {
    match args.output() {
        Output::Stdout => {
            if !args.no_anim && io::stdout().is_terminal() {
                animate(&pass);
            } else if io::stdout().is_terminal() {
                println!("{pass}");
            } else {
                print!("{pass}");
            }
        }
        Output::Clipboard => {
            copy_to_clipboard(pass)?;
        }
        Output::Store(key) => {
            let location = par_core::resolve_new_location(store, key)?;
            if let Some(parent) = location.parent() {
                fs::create_dir_all(parent)?;
            }
            Backend::from(args.backend).encrypt(pass.as_bytes(), store, &location)?;
            if !args.no_git {
                git::add(store, key)?;
            }
            copy_to_clipboard(pass)?;
        }
    }
    Ok(())
}

fn copy_to_clipboard(pass: String) -> anyhow::Result<()> {
    clear_clipboard::clear_in_new_process(pass.as_str(), 10)?;
    ClipboardContext::new()
        .and_then(|mut ctx| ctx.set_contents(pass))
        .map_err(|_| anyhow!("Could not access clipboard"))?;
    Ok(())
}

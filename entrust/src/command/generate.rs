use anyhow::anyhow;
use clap::{Args, ValueEnum};
use copypasta::{ClipboardContext, ClipboardProvider};
use entrust_core::{Backend, generate_passphrase, generate_password};
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::animation::animate;
use crate::command::{BackendValueEnum, clear_clipboard};
use entrust_core::git;

pub(crate) const ABOUT: &str = "Generate a random password";

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[arg(default_value = "phrase")]
    pub(super) r#type: Type,
    /// Copy the generated password to the clipboard
    #[arg(short, long)]
    pub(super) clipboard: bool,
    /// Clear the clipboard after the given number of seconds.
    /// Pass 0 to disable clearing
    #[arg(short = 'd', long, default_value = "10")]
    pub(super) clear_clipboard_delay: u32,
    /// Encrypt and store the generated password under the given key
    #[arg(short, long, value_name = "KEY")]
    pub(super) store: Option<String>,
    /// Length of the password (default: 7 words for type phrase; 20 characters for type word)
    #[arg(short, long)]
    length: Option<u8>,
    /// Word separator for type phrase
    #[arg(long = "sep", default_value = " ")]
    pub(super) separator: String,
    /// Choose gpg or age for en-/decryption
    #[arg(short, long, value_enum, default_value_t = BackendValueEnum::Age)]
    pub(super) backend: BackendValueEnum,
    /// Skip the flashy animation when printing to stdout
    #[arg(short, long)]
    pub(super) no_anim: bool,
    /// Do not add the file to the git repository if one exists (only effective with --store)
    #[arg(long)]
    pub(super) no_git: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub(super) enum Type {
    Phrase,
    Word,
}

impl GenerateArgs {
    pub(super) fn length(&self) -> u8 {
        self.length.unwrap_or(match self.r#type {
            Type::Phrase => 7,
            Type::Word => 20,
        })
    }

    pub(crate) fn needs_backend(&self) -> Option<Backend> {
        if self.store.is_some() {
            Some(self.backend.into())
        } else {
            None
        }
    }
}

pub fn run(store: PathBuf, args: GenerateArgs) -> anyhow::Result<()> {
    let pass = match &args.r#type {
        Type::Phrase => generate_passphrase(args.length(), &args.separator),
        Type::Word => generate_password(args.length()),
    };
    output(&store, args, pass)?;
    Ok(())
}

fn output(store: &Path, args: GenerateArgs, pass: String) -> anyhow::Result<()> {
    if !args.clipboard && args.store.is_none() {
        if !args.no_anim && io::stdout().is_terminal() {
            animate(&pass);
        } else if io::stdout().is_terminal() {
            println!("{pass}");
        } else {
            print!("{pass}");
        }
    }
    if let Some(key) = args.store.as_ref() {
        let location = entrust_core::resolve_new_location(store, key)?;
        if let Some(parent) = location.parent() {
            fs::create_dir_all(parent)?;
        }
        Backend::from(args.backend).encrypt(pass.as_bytes(), store, &location)?;
        if !args.no_git {
            git::add(store, key)?;
        }
    }
    if args.clipboard && args.clear_clipboard_delay > 0 {
        copy_to_clipboard(pass, args.clear_clipboard_delay)?;
    }
    Ok(())
}

fn copy_to_clipboard(pass: String, clear_delay_seconds: u32) -> anyhow::Result<()> {
    clear_clipboard::clear_in_new_process(pass.as_str(), clear_delay_seconds)?;
    ClipboardContext::new()
        .and_then(|mut ctx| ctx.set_contents(pass))
        .map_err(|_| anyhow!("Could not access clipboard"))?;
    Ok(())
}

use anyhow::anyhow;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::{fs, io};

use copypasta::{ClipboardContext, ClipboardProvider};
use rand::prelude::SliceRandom;

use crate::command::generate::animation::animate;
use crate::command::generate::parse::{GenerateArgs, Output, Type};
use crate::generated::wordlist::WORDLIST;
use crate::{git, resolve};

mod animation;
pub mod parse;

pub(crate) const ABOUT: &str = "Generate a random password";

const PRINTABLE_ASCII: &str = r#"!"$#%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#;

pub fn run(store: PathBuf, args: GenerateArgs) -> anyhow::Result<()> {
    let pass = generate_pass(&args)?;
    output(&store, args, pass)?;
    Ok(())
}

fn generate_pass(args: &GenerateArgs) -> anyhow::Result<String> {
    let pass = match args.r#type {
        Type::Phrase => {
            let phrase_iterable = (0..args.length()).map(|_| random_word());
            itertools::intersperse(phrase_iterable, &args.separator).collect()
        }
        Type::Word => {
            let word: Vec<u8> = (0..args.length()).map(|_| random_ascii()).collect();
            String::from_utf8(word)?
        }
    };
    Ok(pass)
}

fn output(store: &Path, args: GenerateArgs, pass: String) -> anyhow::Result<()> {
    match args.output.output() {
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
            ClipboardContext::new()
                .and_then(|mut ctx| ctx.set_contents(pass))
                .map_err(|_| anyhow!("Could not access clipboard"))?;
        }
        Output::Store(key) => {
            let location = resolve::resolve_new(store, key)?;
            if let Some(parent) = location.parent() {
                fs::create_dir_all(parent)?;
            }
            args.backend.encrypt(pass.as_bytes(), store, &location)?;
            if !args.no_git {
                git::add(store, key)?;
            }
        }
    }
    Ok(())
}

fn random_word() -> &'static str {
    WORDLIST.choose(&mut rand::thread_rng()).unwrap()
}

fn random_ascii() -> u8 {
    *PRINTABLE_ASCII
        .as_bytes()
        .choose(&mut rand::thread_rng())
        .unwrap()
}

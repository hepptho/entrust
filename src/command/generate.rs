use anyhow::anyhow;
use std::io;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use copypasta::{ClipboardContext, ClipboardProvider};
use rand::prelude::SliceRandom;

use crate::backend::encrypt_with_backend;
use crate::command::generate::animation::animate;
use crate::command::generate::parse::{GenerateArgs, Output, Type};
use crate::error::ParResult;
use crate::generated::wordlist::WORDLIST;
use crate::{git, resolve};

mod animation;
pub mod parse;

pub(crate) const ABOUT: &str = "Generate a random password";

const PRINTABLE_ASCII: &str = r#"!"$#%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"#;

pub fn run(home: PathBuf, args: GenerateArgs) -> ParResult<()> {
    let pass = generate_pass(&args)?;
    output(&home, args, pass)?;
    Ok(())
}

fn generate_pass(args: &GenerateArgs) -> ParResult<String> {
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

fn output(home: &Path, args: GenerateArgs, pass: String) -> ParResult<()> {
    match args.output.output() {
        Output::Stdout => {
            if !args.no_anim && io::stdout().is_terminal() {
                animate(&pass);
            } else if io::stdout().is_terminal() {
                println!("{}", console::style(pass).cyan());
            } else {
                println!("{pass}");
            }
        }
        Output::Clipboard => {
            ClipboardContext::new()
                .and_then(|mut ctx| ctx.set_contents(pass))
                .map_err(|_| anyhow!("Could not access clipboard"))?;
        }
        Output::Store(key) => {
            let location = resolve::resolve_new(home, key)?;
            encrypt_with_backend(&args.backend, pass.as_bytes(), home, &location)?;
            if !args.no_git {
                git::add(home, key)?;
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

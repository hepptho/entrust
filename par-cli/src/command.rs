pub mod add;
pub mod clear_clipboard;
pub mod completions;
pub mod edit;
pub mod generate;
pub mod get;
mod identity;
pub mod r#move;
pub mod remove;
mod shell;

use crate::command::add::AddArgs;
use crate::command::clear_clipboard::ClearClipboardArgs;
use crate::command::completions::CompletionsArgs;
use crate::command::edit::EditArgs;
use crate::command::generate::parse::GenerateArgs;
use crate::command::get::GetArgs;
use crate::command::r#move::MoveArgs;
use crate::command::remove::RemoveArgs;
use crate::tree::print_tree;
use crate::{init, theme};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use color_print::cstr;
use par_core::Backend;
use std::path::PathBuf;
use std::{env, fs};

const ABOUT: &str = cstr!(
    "

  Manage passwords using <bold,#ffb86c>age</> or <bold,#ffb86c>gpg</>
"
);

const LONG_ABOUT: &str = cstr!(
    "

  Manage passwords using <bold,#ffb86c>age</> or <bold,#ffb86c>gpg</>
  Locations of files to en/decrypt can be given relative to the root of the password store
"
);

#[derive(Parser, Debug)]
#[command(author, version = crate::build_info::HASH, propagate_version = true,
about = ABOUT, long_about = LONG_ABOUT, bin_name = bin_name(),
styles = theme::load_clap_theme())]
pub struct ParArgs {
    #[command(subcommand)]
    pub command: Option<ParSubcommand>,
    /// The directory in which the passwords are stored
    #[arg(short, long, env = par_core::PAR_STORE_ENV_VAR, value_name = "DIR", value_parser = parse_store)]
    pub store: PathBuf,
}

fn parse_store(string: &str) -> Result<PathBuf, String> {
    let buf = PathBuf::from(string);
    if buf.is_file() {
        Err(format!("{string} is a file"))
    } else if buf.exists() {
        Ok(buf)
    } else {
        fs::create_dir_all(&buf)
            .map(|_| buf)
            .map_err(|err| err.to_string())
    }
}

fn bin_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|o| o.to_os_string()))
        .and_then(|o| o.into_string().ok())
        .map(|s| s.replace(".exe", ""))
        .unwrap_or_else(|| "par".to_string())
}

#[derive(Subcommand, Debug)]
pub enum ParSubcommand {
    #[command(about = add::ABOUT, long_about = add::LONG_ABOUT, alias = "insert")]
    Add(AddArgs),
    #[command(about = get::ABOUT, long_about = get::LONG_ABOUT, alias = "g")]
    Get(GetArgs),
    #[command(about = edit::ABOUT, long_about = edit::LONG_ABOUT)]
    Edit(EditArgs),
    #[command(about = r#move::ABOUT, alias = "mv")]
    Move(MoveArgs),
    #[command(about = remove::ABOUT, alias = "rm")]
    Remove(RemoveArgs),
    #[command(about = generate::ABOUT, alias = "gen")]
    Generate(GenerateArgs),
    #[command(about = completions::ABOUT)]
    Completions(CompletionsArgs),
    #[command(about = "Print a tree of the password store")]
    Tree,
    #[command(hide = true)]
    ClearClipboard(ClearClipboardArgs),
    #[command(hide = true)]
    Shell,
    #[command(hide = true)]
    Identity,
}

pub fn run(par: ParArgs) -> anyhow::Result<()> {
    init::init(par.command.as_ref(), &par.store)?;

    match par.command {
        Some(ParSubcommand::Add(args)) => add::run(par.store, args),
        Some(ParSubcommand::ClearClipboard(args)) => clear_clipboard::run(args),
        Some(ParSubcommand::Edit(args)) => edit::run(par.store, args),
        Some(ParSubcommand::Generate(args)) => generate::run(par.store, args),
        Some(ParSubcommand::Get(args)) => get::run(par.store, args),
        Some(ParSubcommand::Move(args)) => r#move::run(par.store, args),
        Some(ParSubcommand::Remove(args)) => remove::run(par.store, args),
        Some(ParSubcommand::Tree) => print_tree(&par.store),
        None => {
            ParArgs::command().print_help()?;
            print_tree(&par.store)?;
            Ok(())
        }
        Some(ParSubcommand::Completions(args)) => {
            completions::run(args);
            Ok(())
        }
        Some(ParSubcommand::Shell) => shell::run(),
        Some(ParSubcommand::Identity) => identity::run(),
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum BackendValueEnum {
    Age,
    Gpg,
}

impl From<BackendValueEnum> for Backend {
    fn from(value: BackendValueEnum) -> Self {
        match value {
            BackendValueEnum::Age => Backend::Age,
            BackendValueEnum::Gpg => Backend::Gpg,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        ParArgs::command().debug_assert();
    }
}

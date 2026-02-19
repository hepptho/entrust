pub mod add;
#[cfg(feature = "autotype")]
mod autotype;
mod clip;
pub mod completions;
pub mod edit;
pub mod generate;
pub mod get;
mod git;
mod identity;
pub mod r#move;
pub mod remove;
mod shell;

use crate::command::add::AddArgs;
#[cfg(feature = "autotype")]
use crate::command::autotype::AutotypeArgs;
use crate::command::clip::ClipArgs;
use crate::command::completions::CompletionsArgs;
use crate::command::edit::EditArgs;
use crate::command::generate::GenerateArgs;
use crate::command::get::GetArgs;
use crate::command::git::GitArgs;
use crate::command::r#move::MoveArgs;
use crate::command::remove::RemoveArgs;
use crate::tree::print_tree;
use crate::{init, theme};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use color_print::cstr;
use const_format::formatcp;
use entrust_core::Backend;
use std::path::PathBuf;
use std::{env, fs};

const ABOUT: &str = cstr!(
    "

  Manage passwords using <bold,#ffb86c>age</> or <bold,#ffb86c>gpg</>
"
);

const LONG_ABOUT: &str = formatcp!(
    "{ABOUT}  \
  Locations of files to en/decrypt can be given relative to the root of the password store
"
);

#[derive(Parser, Debug)]
#[command(author, version = crate::build_info::HASH, propagate_version = true,
about = ABOUT, long_about = LONG_ABOUT, bin_name = bin_name(),
styles = theme::load_clap_theme())]
pub struct EntArgs {
    #[command(subcommand)]
    pub command: Option<EntSubcommand>,
    /// The directory in which the encrypted passwords are stored
    #[arg(short, long, env = entrust_core::ENT_STORE_ENV_VAR, value_name = "DIR", value_parser = parse_store)]
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
        .unwrap_or_else(|| "ent".to_string())
}

#[derive(Subcommand, Debug)]
pub enum EntSubcommand {
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
    #[cfg(feature = "autotype")]
    #[command(about = autotype::ABOUT, alias = "type")]
    Autotype(AutotypeArgs),
    #[command(about = completions::ABOUT)]
    Completions(CompletionsArgs),
    #[command(about = "Print a tree of the password store")]
    Tree,
    #[command(about = git::ABOUT)]
    Git(GitArgs),
    #[command(hide = true)]
    Clip(ClipArgs),
    #[command(hide = true)]
    Shell,
    #[command(hide = true)]
    Identity,
}

pub fn run(ent: EntArgs) -> anyhow::Result<()> {
    init::init(ent.command.as_ref(), &ent.store)?;

    match ent.command {
        Some(EntSubcommand::Add(args)) => add::run(ent.store, args),
        Some(EntSubcommand::Clip(args)) => clip::run(args),
        Some(EntSubcommand::Edit(args)) => edit::run(ent.store, args),
        Some(EntSubcommand::Generate(args)) => generate::run(ent.store, args),
        Some(EntSubcommand::Get(args)) => get::run(ent.store, args),
        Some(EntSubcommand::Move(args)) => r#move::run(ent.store, args),
        Some(EntSubcommand::Remove(args)) => remove::run(ent.store, args),
        #[cfg(feature = "autotype")]
        Some(EntSubcommand::Autotype(args)) => autotype::run(ent.store, args),
        Some(EntSubcommand::Tree) => print_tree(&ent.store),
        Some(EntSubcommand::Git(args)) => git::run(ent.store, args),
        None => {
            EntArgs::command().print_help()?;
            print_tree(&ent.store)?;
            Ok(())
        }
        Some(EntSubcommand::Completions(args)) => {
            completions::run(args);
            Ok(())
        }
        Some(EntSubcommand::Shell) => shell::run(),
        Some(EntSubcommand::Identity) => identity::run(),
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
        EntArgs::command().debug_assert();
    }
}

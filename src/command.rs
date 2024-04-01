pub mod add;
pub mod clear_clipboard;
pub mod completions;
pub mod edit;
pub mod generate;
pub mod get;
pub mod r#move;
pub mod remove;

use crate::command::add::AddArgs;
use crate::command::clear_clipboard::ClearClipboardArgs;
use crate::command::completions::CompletionsArgs;
use crate::command::edit::EditArgs;
use crate::command::generate::parse::GenerateArgs;
use crate::command::get::GetArgs;
use crate::command::r#move::MoveArgs;
use crate::command::remove::RemoveArgs;
use crate::error::ParResult;
use crate::store::get_store;
use crate::tree::print_tree;
use crate::{theme, Backend};
use clap::{CommandFactory, Parser, Subcommand};
use color_print::cstr;
use log::debug;
use std::env;
use std::path::Path;

const ABOUT: &str = cstr!(
    "

  Manage passwords using <bold,#ffb86c>age</> or <bold,#ffb86c>gpg</>
"
);

const LONG_ABOUT: &str = cstr!("

  Manage passwords using <bold,#ffb86c>age</> or <bold,#ffb86c>gpg</>
  Locations of files to en/decrypt can be given relative to the root of the password store in <bold,#ffb86c>PAR_HOME</>
"
);

#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, long_about = LONG_ABOUT, propagate_version = true, bin_name = bin_name(),
styles = theme::clap_theme())]
pub struct ParArgs {
    #[command(subcommand)]
    pub command: Option<ParSubcommands>,
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
pub enum ParSubcommands {
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
    #[command(hide = true)]
    ClearClipboard(ClearClipboardArgs),
    #[command(about = completions::ABOUT)]
    Completions(CompletionsArgs),
}

impl ParSubcommands {
    fn needs_init(&self, store: &Path) -> Option<Backend> {
        match self {
            ParSubcommands::Add(args) => args.backend.needs_init(store),
            ParSubcommands::Edit(args) => args.backend.needs_init(store),
            ParSubcommands::Generate(args) => {
                args.needs_backend().and_then(|b| b.needs_init(store))
            }
            _ => None,
        }
    }
}

pub fn run(par: ParArgs) -> ParResult<()> {
    debug!("{par:#?}");

    let store = get_store()?;
    debug!("store: {store:?}");

    init(par.command.as_ref(), &store)?;

    match par.command {
        Some(ParSubcommands::Add(args)) => add::run(store, args),
        Some(ParSubcommands::ClearClipboard(args)) => clear_clipboard::run(args),
        Some(ParSubcommands::Edit(args)) => edit::run(store, args),
        Some(ParSubcommands::Generate(args)) => generate::run(store, args),
        Some(ParSubcommands::Get(args)) => get::run(store, args),
        Some(ParSubcommands::Move(args)) => r#move::run(store, args),
        Some(ParSubcommands::Remove(args)) => remove::run(store, args),
        Some(ParSubcommands::Completions(args)) => {
            completions::run(args);
            Ok(())
        }
        None => {
            ParArgs::command().print_help()?;
            print_tree(&store)?;
            Ok(())
        }
    }
}

fn init(subcommand: Option<&ParSubcommands>, store: &Path) -> ParResult<()> {
    let needs_init = subcommand.and_then(|c| c.needs_init(store));
    if let Some(backend) = needs_init {
        backend.create_recipient_file_if_not_present(store)?;
    }
    Ok(())
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

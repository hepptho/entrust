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
use crate::theme;
use clap::{Parser, Subcommand};
use color_print::cstr;
use std::env;

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
pub struct Par {
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

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Par::command().debug_assert();
    }
}

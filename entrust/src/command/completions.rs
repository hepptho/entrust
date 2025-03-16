use crate::command::EntArgs;
use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};
use std::io;

pub(super) const ABOUT: &str = "Generate shell completions";

const NUSHELL_COMPLETIONS: &str = include_str!("../../extern-ent.nu");

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    shell: Option<Shell>,
    #[arg(short, long, conflicts_with = "shell")]
    nushell: bool,
}

pub fn run(args: CompletionsArgs) {
    let mut cmd = EntArgs::command();
    let bin_name = cmd.get_bin_name().unwrap_or(cmd.get_name()).to_string();
    if let Some(shell) = args.shell {
        generate(shell, &mut cmd, &bin_name, &mut io::stdout());
    }
    if args.nushell {
        print!("{NUSHELL_COMPLETIONS}",)
    }
}

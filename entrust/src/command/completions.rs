use crate::command::EntArgs;
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

pub(super) const ABOUT: &str = "Generate shell completions";

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    shell: Shell,
}

pub fn run(args: CompletionsArgs) {
    let mut cmd = EntArgs::command();
    let bin_name = cmd.get_bin_name().unwrap_or(cmd.get_name()).to_string();
    generate(args.shell, &mut cmd, bin_name, &mut io::stdout());
}

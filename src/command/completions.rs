use crate::command::Par;
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

pub(super) const ABOUT: &str = "Generate shell completions";

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    shell: Shell,
}

pub fn run(args: CompletionsArgs) {
    let cmd = Par::command();
    generate(
        args.shell,
        &mut cmd.clone(),
        cmd.get_bin_name().unwrap_or(cmd.get_name()),
        &mut io::stdout(),
    );
}

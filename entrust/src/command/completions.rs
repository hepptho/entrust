use crate::command::EntArgs;
use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::generate;
use std::io;

pub(super) const ABOUT: &str = "Generate shell completions";

const NUSHELL_COMPLETIONS: &str = include_str!("../../extern-ent.nu");

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    shell: Shell,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
    Nushell,
}

pub fn run(args: CompletionsArgs) {
    let mut cmd = EntArgs::command();
    let bin_name = cmd.get_bin_name().unwrap_or(cmd.get_name()).to_string();
    if args.shell == Shell::Nushell {
        print!("{NUSHELL_COMPLETIONS}",)
    } else {
        if let Ok(shell) = clap_complete::Shell::try_from(args.shell) {
            generate(shell, &mut cmd, &bin_name, &mut io::stdout());
        }
    }
}

impl TryFrom<Shell> for clap_complete::Shell {
    type Error = ();

    fn try_from(value: Shell) -> Result<Self, Self::Error> {
        match value {
            Shell::Bash => Ok(clap_complete::Shell::Bash),
            Shell::Elvish => Ok(clap_complete::Shell::Elvish),
            Shell::Fish => Ok(clap_complete::Shell::Fish),
            Shell::PowerShell => Ok(clap_complete::Shell::PowerShell),
            Shell::Zsh => Ok(clap_complete::Shell::Zsh),
            Shell::Nushell => Err(()),
        }
    }
}

use crate::command;
use crate::command::{bin_name, ParArgs};
use anyhow::anyhow;
use clap::Parser;
use par_core::age;
use par_dialog::dialog::Dialog;
use par_dialog::input::prompt::Prompt;
use par_dialog::input::InputDialog;

pub fn run() -> anyhow::Result<()> {
    age::initialize_identity()?;
    let bin_name = bin_name();
    loop {
        let input = InputDialog::default()
            .with_prompt(Prompt::inline("par ❯ "))
            .run()?;
        if input.is_empty() {
            continue;
        }
        if input == "q" || input == "quit" {
            break;
        }
        if input == "c" || input == "copy" {
            command::run(ParArgs::parse_from([bin_name.as_str(), "get", "-c"]))?
        } else {
            parse_and_run(input, bin_name.as_str());
        }
    }
    Ok(())
}

fn parse_and_run(input: String, bin_name: &str) {
    shlex::split(input.as_str())
        .ok_or(anyhow!("Invalid input"))
        .and_then(|mut args| {
            args.insert(0, bin_name.to_string());
            ParArgs::try_parse_from(args).map_err(anyhow::Error::from)
        })
        .and_then(command::run)
        .unwrap_or_else(|err| eprintln!("{err}"));
}

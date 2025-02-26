use crate::alias::apply_aliases;
use crate::command;
use crate::command::{EntArgs, bin_name};
use crate::theme::chevron_prompt;
use anyhow::anyhow;
use clap::Parser;
use entrust_core::age;
use entrust_dialog::dialog::Dialog;
use entrust_dialog::input::InputDialog;
use entrust_dialog::input::prompt::Prompt;

pub fn run() -> anyhow::Result<()> {
    age::identity::get_identity()?;
    loop {
        let input = InputDialog::default()
            .with_prompt(Prompt::inline(chevron_prompt!("ent")))
            .with_completions(vec![
                "add".into(),
                "copy".into(),
                "get".into(),
                "edit".into(),
                "move".into(),
                "remove".into(),
                "generate".into(),
                "tree".into(),
            ])
            .run()?;
        if input.is_empty() {
            continue;
        }
        if input == "q" || input == "quit" {
            break;
        }
        parse_and_run(input);
    }
    Ok(())
}

fn parse_and_run(input: String) {
    shlex::split(input.as_str())
        .ok_or(anyhow!("Invalid input"))
        .and_then(|mut args| {
            args.insert(0, bin_name());
            apply_aliases(&mut args);
            EntArgs::try_parse_from(args).map_err(anyhow::Error::from)
        })
        .and_then(command::run)
        .unwrap_or_else(|err| eprintln!("{err}"));
}

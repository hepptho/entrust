use crate::command::ParSubcommand;
use crate::theme::{CHEVRON, DIALOG_THEME};
use par_core::Backend;
use par_dialog::dialog::Dialog;
use par_dialog::input::prompt::Prompt;
use par_dialog::input::InputDialog;
use std::fs;
use std::ops::Deref;
use std::path::Path;

pub fn init(subcommand: Option<&ParSubcommand>, store: &Path) -> anyhow::Result<()> {
    let needs_init = subcommand.and_then(|c| needs_init(c, store));
    if let Some(backend) = needs_init {
        create_recipient_file_if_not_present(backend, store)?;
    }
    Ok(())
}

fn needs_init(subcommand: &ParSubcommand, store: &Path) -> Option<Backend> {
    match subcommand {
        ParSubcommand::Add(args) => Backend::from(args.backend).needs_init(store),
        ParSubcommand::Edit(args) => Backend::from(args.backend).needs_init(store),
        ParSubcommand::Generate(args) => args.needs_backend().and_then(|b| b.needs_init(store)),
        _ => None,
    }
}

fn create_recipient_file_if_not_present(backend: Backend, store: &Path) -> anyhow::Result<()> {
    let file = store.join(backend.recipient_file_name());
    if file.exists() {
        return Ok(());
    }
    let prompt = format!(
        "{} recipient for which the file should be created {} ",
        backend.display_name(),
        CHEVRON
    );
    // TODO leaking is fine here but maybe we can do better
    let recipient = InputDialog::default()
        .with_prompt(Prompt::inline(prompt.leak()))
        .with_theme(DIALOG_THEME.deref())
        .run()?;
    fs::write(file, recipient.as_bytes())?;
    Ok(())
}

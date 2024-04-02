use crate::command::ParSubcommand;
use crate::theme::INQUIRE_RENDER_CONFIG;
use crate::Backend;
use inquire::Text;
use std::fs;
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
        ParSubcommand::Add(args) => args.backend.needs_init(store),
        ParSubcommand::Edit(args) => args.backend.needs_init(store),
        ParSubcommand::Generate(args) => args.needs_backend().and_then(|b| b.needs_init(store)),
        _ => None,
    }
}

fn create_recipient_file_if_not_present(backend: Backend, store: &Path) -> anyhow::Result<()> {
    let file = store.join(backend.recipient_file_name());
    if file.exists() {
        return Ok(());
    }
    let recipient: String = Text::new(
        format!(
            "{} recipient for which the file should be created ❯",
            backend.display_name()
        )
        .as_str(),
    )
    .with_render_config(*INQUIRE_RENDER_CONFIG)
    .prompt()?;
    fs::write(file, recipient.as_bytes())?;
    Ok(())
}

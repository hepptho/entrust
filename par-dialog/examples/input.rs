use par_dialog::dialog::Dialog;
use par_dialog::input::prompt::Prompt;
use par_dialog::input::InputDialog;

fn main() -> anyhow::Result<()> {
    InputDialog::default()
        .with_prompt(Prompt::inline("Enter some text :) "))
        .run()?;
    Ok(())
}

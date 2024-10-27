use entrust_dialog::dialog::Dialog;
use entrust_dialog::input::prompt::Prompt;
use entrust_dialog::input::InputDialog;

fn main() -> anyhow::Result<()> {
    InputDialog::default()
        .with_prompt(Prompt::inline("Enter some text :) "))
        .run()?;
    Ok(())
}

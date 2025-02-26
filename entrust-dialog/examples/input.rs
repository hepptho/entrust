use entrust_dialog::dialog::Dialog;
use entrust_dialog::input::InputDialog;
use entrust_dialog::input::prompt::Prompt;

fn main() -> anyhow::Result<()> {
    InputDialog::default()
        .with_prompt(Prompt::inline("Enter some text :) "))
        .run()?;
    Ok(())
}

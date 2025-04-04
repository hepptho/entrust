use entrust_dialog::dialog::Dialog;
use entrust_dialog::yes_no::YesNoDialog;

fn main() -> anyhow::Result<()> {
    let result = YesNoDialog::default().with_message("Yes or no?").run()?;
    let display = if result { "yes" } else { "no" };
    println!("You chose: {display}");
    Ok(())
}

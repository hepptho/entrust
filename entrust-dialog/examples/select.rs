use entrust_dialog::dialog::Dialog;
use entrust_dialog::select::SelectDialog;

fn main() -> anyhow::Result<()> {
    let result = SelectDialog::new(vec!["one".into(), "two".into(), "three".into()]).run()?;
    println!("{}", result.unwrap_or("<none>".into()));
    Ok(())
}

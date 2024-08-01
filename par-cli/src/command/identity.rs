use par_core::age;
use std::io;
use std::io::IsTerminal;

pub fn run() -> anyhow::Result<()> {
    let age_identity = String::from_utf8(age::identity::get_identity()?.clone())?;
    print!("{age_identity}");
    if io::stdout().is_terminal() {
        println!();
    }
    Ok(())
}

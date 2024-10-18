use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::thread;
use std::time::Duration;

pub fn alt_tab() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Alt, Direction::Press)?;
    enigo.key(Key::Tab, Direction::Click)?;
    enigo.key(Key::Alt, Direction::Release)?;
    Ok(())
}

pub fn tab() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Tab, Direction::Click)?;
    Ok(())
}

pub fn enter() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Return, Direction::Click)?;
    Ok(())
}

pub fn text(text: impl AsRef<str>) -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    thread::sleep(Duration::from_millis(500));
    // workaround for https://github.com/enigo-rs/enigo/issues/303
    for (index, line) in text.as_ref().split('\n').enumerate() {
        if index != 0 {
            enigo.text("\n")?;
        }
        enigo.text(line)?;
    }
    Ok(())
}

use crate::dialog;
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use const_format::formatcp;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use entrust_core::{Backend, resolve_existing_location};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub(super) const ABOUT: &str = "Autotype into the previously active window";

const HELP: &str = "One or more keys, separated by a colon, and optionally {tab} or {enter}";

const LONG_HELP_ADDITION: &str = cstr!(
    "\
A prompt will be displayed for omitted keys. Examples:
<bold>something/user:{tab}:something/pass:{enter}</>
<bold>:{tab}::{enter}</>"
);

const LONG_HELP: &str = formatcp!("{HELP}\n{LONG_HELP_ADDITION}");

#[derive(Args, Debug)]
pub struct AutotypeArgs {
    #[arg(help = HELP, long_help = LONG_HELP, default_value = "", hide_default_value = true)]
    segments: String,
}

#[derive(Debug)]
enum Segment {
    Pass(String),
    Tab,
    Enter,
}

pub fn run(store: PathBuf, args: AutotypeArgs) -> anyhow::Result<()> {
    let segments = parse_segments(args.segments.as_str(), store.as_path())?;
    if !segments.is_empty() {
        autotype_alt_tab()?;
    }
    for segment in segments {
        match segment {
            Segment::Pass(pass) => autotype_text(pass)?,
            Segment::Tab => autotype_tab()?,
            Segment::Enter => autotype_enter()?,
        }
    }
    Ok(())
}

fn parse_segments(string: &str, store: &Path) -> anyhow::Result<Vec<Segment>> {
    string.split(':').map(|s| parse_segment(s, store)).collect()
}

fn parse_segment(string: &str, store: &Path) -> anyhow::Result<Segment> {
    match string {
        "{tab}" => Ok(Segment::Tab),
        "{enter}" => Ok(Segment::Enter),
        s if s.starts_with("{") && s.ends_with("}") => Err(anyhow!("Unsupported key press: {s}")),
        "" => {
            let key = store.join(dialog::select_existing_key(store)?);
            let pass = Backend::decrypt(&key)?;
            Ok(Segment::Pass(pass))
        }
        s => {
            let key = resolve_existing_location(store, s, false)?;
            let pass = Backend::decrypt(&key)?;
            Ok(Segment::Pass(pass))
        }
    }
}

fn autotype_alt_tab() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Alt, Direction::Press)?;
    enigo.key(Key::Tab, Direction::Click)?;
    enigo.key(Key::Alt, Direction::Release)?;
    Ok(())
}

fn autotype_tab() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Tab, Direction::Click)?;
    Ok(())
}

fn autotype_enter() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Return, Direction::Click)?;
    Ok(())
}

fn autotype_text(text: impl AsRef<str>) -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    thread::sleep(Duration::from_millis(500));
    // workaround for https://github.com/enigo-rs/enigo/issues/303
    for line in itertools::intersperse(text.as_ref().split('\n'), "\n") {
        enigo.text(line)?;
    }
    Ok(())
}

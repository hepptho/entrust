use crate::{autotype, dialog};
use anyhow::anyhow;
use clap::Args;
use color_print::cstr;
use const_format::formatcp;
use par_core::{resolve_existing_location, Backend};
use std::path::{Path, PathBuf};

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
        autotype::alt_tab()?;
    }
    for segment in segments {
        match segment {
            Segment::Pass(pass) => autotype::text(pass)?,
            Segment::Tab => autotype::tab()?,
            Segment::Enter => autotype::enter()?,
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

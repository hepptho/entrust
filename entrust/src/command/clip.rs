use crate::theme::color;
use arboard::Clipboard;
use clap::{Args, Subcommand};
use color_print::cprintln;
use std::borrow::Cow;
use std::io::{IsTerminal, Read, Write};
use std::process::{Command, Stdio};
use std::{env, io, thread};

#[derive(Args, Debug)]
pub struct ClipArgs {
    #[command(subcommand)]
    pub command: ClipSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ClipSubcommand {
    Copy,
    Clear {
        #[arg(short, long, default_value_t = 0)]
        delay_seconds: u64,
    },
}

pub fn run(args: ClipArgs) -> anyhow::Result<()> {
    match args.command {
        ClipSubcommand::Copy => copy_stdin(),
        ClipSubcommand::Clear { delay_seconds } => clear(delay_seconds, true),
    }
}

pub fn copy(content: Cow<'_, str>) -> anyhow::Result<()> {
    if cfg!(target_os = "linux") {
        copy_in_new_process(&content)
    } else {
        copy_now(content)
    }
}

fn copy_stdin() -> anyhow::Result<()> {
    if io::stdin().is_terminal() {
        return Ok(());
    };
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;

    #[cfg(target_os = "linux")]
    linux::copy_wait(stdin.into())?;
    #[cfg(not(target_os = "linux"))]
    copy_now(stdin.into())?;

    Ok(())
}

fn copy_now(content: Cow<str>) -> anyhow::Result<()> {
    Clipboard::new()?.set_text(content)?;
    Ok(())
}

#[cfg(target_os = "linux")]
mod linux {
    use arboard::{Clipboard, SetExtLinux};
    use std::borrow::Cow;
    use std::time::{Duration, Instant};

    pub fn copy_wait(content: Cow<str>) -> anyhow::Result<()> {
        Clipboard::new()?
            .set()
            .wait_until(Instant::now() + Duration::from_secs(5))
            .text(content)?;
        Ok(())
    }
}

fn copy_in_new_process(content: &str) -> anyhow::Result<()> {
    let current_exe = env::current_exe()?;
    let mut child = Command::new(current_exe)
        .arg("clip")
        .arg("copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(content.as_bytes())?;
    drop(stdin);
    Ok(())
}

fn clear(delay_seconds: u64, only_if_stdin_matches: bool) -> anyhow::Result<()> {
    if delay_seconds > 0 {
        thread::sleep(std::time::Duration::from_secs(delay_seconds));
    };
    if !only_if_stdin_matches || io::stdin().is_terminal() {
        return clear_now();
    }
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;
    let stdin_matches_current = || {
        Clipboard::new()
            .and_then(|mut c| c.get_text())
            .is_ok_and(|current| {
                let matches = current.as_str() == stdin.trim_end();
                #[cfg(feature = "tracing")]
                trace_match(stdin.as_str(), current.as_str(), matches);
                matches
            })
    };
    if stdin.is_empty() || stdin_matches_current() {
        clear_now()?;
    }
    Ok(())
}

#[cfg(feature = "tracing")]
fn trace_match(stdin: &str, current: &str, matches: bool) {
    tracing::info!("stdin matches clipboard: {matches}");
    if !matches {
        let matches_trimmed = current.trim() == stdin.trim();
        tracing::info!("stdin matches trimmed clipboard: {matches_trimmed}");
    }
}

fn clear_now() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing::info!("clearing clipboard");
    Clipboard::new()?.clear()?;
    Ok(())
}

pub fn clear_in_new_process(content: &str, delay_seconds: u64) -> anyhow::Result<()> {
    let current_exe = env::current_exe()?;
    let mut child = Command::new(current_exe)
        .arg("clip")
        .arg("clear")
        .arg("-d")
        .arg(delay_seconds.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(content.as_bytes())?;
    drop(stdin);
    if color() {
        cprintln!(
            "<bright-black>The clipboard will be cleared in {delay_seconds}s if it has not changed."
        );
    } else {
        println!("The clipboard will be cleared in {delay_seconds}s if it has not changed.");
    };

    Ok(())
}

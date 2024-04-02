use anyhow::anyhow;
use clap::Args;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, thread};

#[derive(Args, Debug)]
pub struct ClearClipboardArgs {
    #[arg(short, long)]
    delay_seconds: Option<u32>,
}

pub fn run(args: ClearClipboardArgs) -> anyhow::Result<()> {
    if let Some(delay_seconds) = args.delay_seconds {
        thread::sleep(Duration::from_secs(delay_seconds as u64))
    }
    ClipboardContext::new()
        .and_then(|mut ctx| ctx.set_contents(" ".to_string()))
        .map_err(|_| anyhow!("Could not access clipboard"))
}

pub(crate) fn clear_in_new_process(delay_seconds: u32) -> anyhow::Result<()> {
    let current_exe = env::current_exe()?;
    Command::new(current_exe)
        .arg("clear-clipboard")
        .arg("-d")
        .arg(delay_seconds.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

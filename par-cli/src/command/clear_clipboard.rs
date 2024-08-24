use anyhow::anyhow;
use clap::Args;
use color_print::cprintln;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{IsTerminal, Read, Write};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, io, thread};

#[derive(Args, Debug)]
pub struct ClearClipboardArgs {
    #[arg(short, long)]
    delay_seconds: Option<u32>,
}

pub fn run(args: ClearClipboardArgs) -> anyhow::Result<()> {
    if let Some(delay_seconds) = args.delay_seconds {
        thread::sleep(Duration::from_secs(delay_seconds as u64))
    }
    if io::stdin().is_terminal() {
        return clear_now();
    }
    let mut stdin = String::new();
    match io::stdin().read_to_string(&mut stdin) {
        Ok(_) => {
            if stdin.is_empty() {
                clear_now()
            } else if ClipboardContext::new()
                .and_then(|mut ctx| ctx.get_contents())
                .is_ok_and(|current| current.as_str() == stdin.trim_end())
            {
                clear_now()
            } else {
                Ok(())
            }
        }
        Err(_) => clear_now(),
    }
}

fn clear_now() -> anyhow::Result<()> {
    ClipboardContext::new()
        .and_then(|mut ctx| ctx.set_contents(" ".to_string()))
        .map_err(|_| anyhow!("Could not access clipboard"))
}

pub(crate) fn clear_in_new_process(content: &str, delay_seconds: u32) -> anyhow::Result<()> {
    let current_exe = env::current_exe()?;
    let mut child = Command::new(current_exe)
        .arg("clear-clipboard")
        .arg("-d")
        .arg(delay_seconds.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    stdin.write(content.as_bytes())?;
    drop(stdin);
    cprintln!(
        "<bright-black>The clipboard will be cleared in {delay_seconds}s if it has not changed."
    );
    Ok(())
}

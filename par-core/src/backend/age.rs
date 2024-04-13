mod identity;

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, io};

use crate::age::identity::{identity_file, read_identity_or_get_cached};
use anyhow::anyhow;
use log::debug;

pub const RECIPIENT_FILE_NAME: &str = ".age-id";

pub fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> anyhow::Result<()> {
    let mut child = Command::new("age")
        .arg("--encrypt")
        .arg("--armor")
        .arg("--recipient")
        .arg(recipient)
        .arg("--output")
        .arg(out_path.as_os_str())
        .stdin(Stdio::piped())
        .spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    io::copy(content, &mut child_stdin)?;
    drop(child_stdin);
    child.wait()?;
    Ok(())
}

pub fn decrypt(path: &Path) -> anyhow::Result<String> {
    let identity_from_stdin = read_identity_or_get_cached()?;

    let cmd_identity = if identity_from_stdin.is_some() {
        "-".to_string()
    } else {
        identity_file().ok_or(anyhow!("No age identity provided"))?
    };
    debug!("cmd_identity: {cmd_identity}");
    let mut child = Command::new("age")
        .arg("--decrypt")
        .arg("--identity")
        .arg(cmd_identity)
        .arg(path.as_os_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let content = if let Some(identity) = identity_from_stdin {
        identity.clone()
    } else {
        vec![]
    };
    io::copy(&mut content.as_slice(), &mut child_stdin)?;
    drop(child_stdin);
    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}

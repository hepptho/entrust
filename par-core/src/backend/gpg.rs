use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

pub const RECIPIENT_FILE_NAME: &str = ".gpg-id";

pub fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> anyhow::Result<()> {
    let mut child = Command::new("gpg")
        .arg("--encrypt")
        .arg("--armor")
        .arg("--quiet")
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
    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg("--quiet")
        .arg(path.as_os_str())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

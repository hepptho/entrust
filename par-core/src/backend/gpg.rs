use crate::backend::{exit_status_to_result, output_to_result};
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
    let exit_status = child.wait()?;
    exit_status_to_result(exit_status, "gpg")
}

pub fn decrypt(path: &Path) -> anyhow::Result<String> {
    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg("--quiet")
        .arg(path.as_os_str())
        .stdin(Stdio::inherit())
        .output()?;
    output_to_result(output)
}

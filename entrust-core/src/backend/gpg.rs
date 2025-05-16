use crate::backend::{exit_status_to_result, output_to_result};
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

pub const RECIPIENT_FILE_NAME: &str = ".gpg-id";

pub fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> anyhow::Result<()> {
    let (in_read, mut in_write) = io::pipe()?;
    let child = Command::new("gpg")
        .arg("--encrypt")
        .arg("--armor")
        .arg("--quiet")
        .arg("--recipient")
        .arg(recipient)
        .arg("--output")
        .arg(out_path.as_os_str())
        .stdin(in_read)
        .spawn()?;
    io::copy(content, &mut in_write)?;
    drop(in_write);
    let exit_status = child.wait_with_output()?.status;
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

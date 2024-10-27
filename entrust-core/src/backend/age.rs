#[cfg(feature = "agent")]
mod agent;
pub mod identity;
#[cfg(not(feature = "agent"))]
mod no_agent;

use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::age::identity::get_identity;
use crate::backend::{exit_status_to_result, output_to_result};

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
    let exit_status = child.wait()?;
    exit_status_to_result(exit_status, "age")
}

pub fn decrypt(path: &Path) -> anyhow::Result<String> {
    let identity = get_identity()?;
    let mut child = Command::new("age")
        .arg("--decrypt")
        .arg("--identity")
        .arg("-")
        .arg(path.as_os_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut child_stdin = child.stdin.take().unwrap();
    io::copy(&mut identity.as_slice(), &mut child_stdin)?;
    drop(child_stdin);
    let output = child.wait_with_output()?;
    output_to_result(output)
}

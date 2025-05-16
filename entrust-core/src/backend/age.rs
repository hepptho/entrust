#[cfg(feature = "agent")]
mod agent;
pub mod identity;
#[cfg(not(feature = "agent"))]
mod no_agent;

use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::age::identity::get_identity;
use crate::backend::{exit_status_to_result, output_to_result};

pub const RECIPIENT_FILE_NAME: &str = ".age-id";

pub fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> anyhow::Result<()> {
    let (in_read, mut in_write) = io::pipe()?;
    let mut child = Command::new("age")
        .arg("--encrypt")
        .arg("--armor")
        .arg("--recipient")
        .arg(recipient)
        .arg("--output")
        .arg(out_path.as_os_str())
        .stdin(in_read)
        .spawn()?;
    io::copy(content, &mut in_write)?;
    drop(in_write);
    let exit_status = child.wait()?;
    exit_status_to_result(exit_status, "age")
}

pub fn decrypt(path: &Path) -> anyhow::Result<String> {
    let identity = get_identity()?;
    let (in_read, mut in_write) = io::pipe()?;
    let child = Command::new("age")
        .arg("--decrypt")
        .arg("--identity")
        .arg("-")
        .arg(path.as_os_str())
        .stdin(in_read)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    in_write.write_all(identity.as_slice())?;
    drop(in_write);
    let output = child.wait_with_output()?;
    output_to_result(output)
}

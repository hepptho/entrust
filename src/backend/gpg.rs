use crate::backend::Backend;
use crate::error::ParResult;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

const RECIPIENT_FILE_NAME: &str = ".gpg-id";

pub struct Gpg {}

impl Backend for Gpg {
    fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> ParResult<()> {
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

    fn decrypt(path: &Path) -> ParResult<String> {
        let output = Command::new("gpg")
            .arg("--decrypt")
            .arg("--quiet")
            .arg(path.as_os_str())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
        Ok(String::from_utf8(output.stdout)?)
    }

    fn recipient_file_name() -> &'static str {
        RECIPIENT_FILE_NAME
    }
}

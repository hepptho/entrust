use crate::backend::Backend;
use crate::error::ParResult;
use anyhow::anyhow;
use log::debug;
use std::io::{IsTerminal, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, io};

const RECIPIENT_FILE_NAME: &str = ".age-id";

pub struct Age {}

impl Age {
    fn identity_file() -> Option<String> {
        env::var("AGE_IDENTITY").ok()
    }
}

impl Backend for Age {
    fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> ParResult<()> {
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

    fn decrypt(path: &Path) -> ParResult<String> {
        let has_input = !io::stdin().is_terminal();
        debug!("has_input: {has_input}");
        let cmd_identity = if has_input {
            "-".to_string()
        } else {
            Age::identity_file().ok_or(anyhow!("No age identity provided"))?
        };
        debug!("cmd_identity: {cmd_identity}");
        let output = Command::new("age")
            .arg("--decrypt")
            .arg("--identity")
            .arg(cmd_identity)
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

use crate::backend::is_age_encrypted;
use anyhow::anyhow;
use std::io::{IsTerminal, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::{env, fs, io};

static IDENTITY: OnceLock<anyhow::Result<Vec<u8>>> = OnceLock::new();

pub fn get_identity() -> anyhow::Result<&'static Vec<u8>> {
    IDENTITY
        .get_or_init(|| {
            if !io::stdin().is_terminal() {
                read_identity_from_stdin()
            } else if let Some(identity_file) = identity_file() {
                read_identity_from_file(identity_file)
            } else {
                Err(anyhow!(""))
            }
        })
        .as_ref()
        .map_err(|err| anyhow!(err))
}

fn read_identity_from_stdin() -> anyhow::Result<Vec<u8>> {
    let mut identity = Vec::new();
    io::stdin().read_to_end(&mut identity)?;
    Ok(identity)
}

fn read_identity_from_file(identity_file: String) -> anyhow::Result<Vec<u8>> {
    if is_age_encrypted(Path::new(identity_file.as_str()))? {
        decrypt_identity_file(identity_file)
    } else {
        fs::read(identity_file).map_err(anyhow::Error::from)
    }
}

fn decrypt_identity_file(identity_file: String) -> anyhow::Result<Vec<u8>> {
    let output = Command::new("age")
        .arg("--decrypt")
        .arg(identity_file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(anyhow!("age exited with an error"))
    }
}

fn identity_file() -> Option<String> {
    env::var("AGE_IDENTITY").ok()
}

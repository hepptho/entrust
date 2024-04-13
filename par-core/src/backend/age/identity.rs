use std::io::{IsTerminal, Read};
use std::sync::OnceLock;
use std::{env, io};

static IDENTITY: OnceLock<Option<Vec<u8>>> = OnceLock::new();

fn read_identity_from_stdin() -> anyhow::Result<Option<Vec<u8>>> {
    if io::stdin().is_terminal() {
        Ok(None)
    } else {
        let mut identity = Vec::new();
        io::stdin().read_to_end(&mut identity)?;
        Ok(Some(identity))
    }
}

pub fn read_identity_or_get_cached() -> anyhow::Result<Option<&'static Vec<u8>>> {
    let identity_from_stdin = if let Some(id) = IDENTITY.get() {
        id
    } else {
        let id = read_identity_from_stdin()?;
        IDENTITY.get_or_init(|| id)
    };
    Ok(identity_from_stdin.as_ref())
}

pub fn identity_file() -> Option<String> {
    env::var("AGE_IDENTITY").ok()
}

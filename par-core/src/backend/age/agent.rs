use crate::age::identity::read_identity;
use std::borrow::Cow;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use std::{env, io};

pub fn get_identity() -> anyhow::Result<Vec<u8>> {
    match get_identity_from_agent() {
        Err(e) if e.kind() == ErrorKind::NotFound => start_agent()?,
        Ok(id) if id.as_slice() == [b'\n'] => {}
        Ok(id) if id.as_slice() == [b'-', b'\n'] => start_agent()?,
        Ok(id) => return Ok(id),
        Err(e) => return Err(e.into()),
    }

    let id = read_identity()?;
    par_agent::client::set_age_identity(
        String::from_utf8(id.clone())?.as_str(),
        env::var("PAR_AGENT_PIN").ok(),
    )?;
    Ok(id)
}

fn get_identity_from_agent() -> io::Result<Vec<u8>> {
    let string = par_agent::client::get_age_identity(env::var("PAR_AGENT_PIN").ok())?;
    Ok(string.into_bytes())
}

fn start_agent() -> io::Result<()> {
    let var = env::var("PAR_AGENT_SECONDS").ok();
    let seconds = var
        .as_ref()
        .and_then(|v| v.parse::<usize>().ok().map(|_| v.as_str()))
        .unwrap_or("600");
    let par_agent_bin = env::var("PAR_AGENT_BIN")
        .map(Cow::Owned)
        .unwrap_or("par-agent".into());
    Command::new(par_agent_bin.as_ref())
        .arg(seconds)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
}

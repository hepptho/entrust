use crate::age::identity::read_identity;
use anyhow::anyhow;
use par_agent::server::GetAgeIdentityResponse;
use std::borrow::Cow;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use std::{env, io};

pub fn get_identity() -> anyhow::Result<Vec<u8>> {
    let result = par_agent::client::get_age_identity(env::var("PAR_AGENT_PIN").ok());
    match result {
        Err(e) if e.kind() == ErrorKind::NotFound => start_agent()?,
        Err(e) => return Err(anyhow!(e)),
        Ok(response) => match response {
            GetAgeIdentityResponse::Ok { identity } => return Ok(identity.into_bytes()),
            GetAgeIdentityResponse::NotSet => start_agent()?,
            GetAgeIdentityResponse::WrongPin => par_agent::client::shutdown_server()?,
        },
    }

    let id = read_identity()?;
    par_agent::client::set_age_identity(
        String::from_utf8(id.clone())?,
        env::var("PAR_AGENT_PIN").ok(),
    )?;
    Ok(id)
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

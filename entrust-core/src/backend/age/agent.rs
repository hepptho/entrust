use crate::age::identity::read_identity;
use anyhow::anyhow;
use entrust_agent::NO_AGENT_ERROR_KIND;
use entrust_agent::server::GetAgeIdentityResponse;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{io, thread};

pub fn get_identity() -> anyhow::Result<Vec<u8>> {
    let result = entrust_agent::client::get_age_identity(entrust_agent::env::agent_pin().ok());
    match result {
        Err(e) if e.kind() == NO_AGENT_ERROR_KIND => start_agent()?,
        Err(e) => return Err(anyhow!(e)),
        Ok(response) => match response {
            GetAgeIdentityResponse::Ok { identity } => return Ok(identity.into_bytes()),
            GetAgeIdentityResponse::NotSet => start_agent()?,
            GetAgeIdentityResponse::WrongPin => entrust_agent::client::shutdown_server()?,
        },
    }

    let id = read_identity()?;
    set_identity(&id)?;
    Ok(id)
}

fn start_agent() -> io::Result<()> {
    let seconds = entrust_agent::env::agent_seconds();
    let ent_agent_bin = entrust_agent::env::agent_bin();
    let mut command = if cfg!(windows) {
        let mut command = Command::new("pwsh");
        command.args(["-C", "Start-Process", ent_agent_bin.as_ref()]);
        command
    } else {
        Command::new(ent_agent_bin.as_ref())
    };
    command
        .arg(seconds.as_ref())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
}

fn set_identity(id: &[u8]) -> anyhow::Result<()> {
    let string = String::from_utf8(id.to_vec())?;
    let result =
        entrust_agent::client::set_age_identity(string, entrust_agent::env::agent_pin().ok());
    match result {
        Err(e) if e.kind() == NO_AGENT_ERROR_KIND => {
            thread::sleep(Duration::from_millis(100));
            entrust_agent::client::set_age_identity(
                String::from_utf8(id.to_vec())?,
                entrust_agent::env::agent_pin().ok(),
            )
        }
        result => result,
    }?;
    Ok(())
}

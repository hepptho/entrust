use crate::age::identity::read_identity;
use anyhow::anyhow;
use entrust_agent::NO_AGENT_ERROR_KIND;
use entrust_agent::server::GetAgeIdentityResponse;
use std::borrow::Cow;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, io, thread};

pub fn get_identity() -> anyhow::Result<Vec<u8>> {
    let result = entrust_agent::client::get_age_identity(env::var("ENT_AGENT_PIN").ok());
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
    let var = env::var("ENT_AGENT_SECONDS").ok();
    let seconds = var
        .as_ref()
        .and_then(|v| v.parse::<usize>().ok().map(|_| v.as_str()))
        .unwrap_or("600");
    let ent_agent_bin = env::var("ENT_AGENT_BIN")
        .map(Cow::Owned)
        .unwrap_or("ent-agent".into());
    let mut command = if cfg!(windows) {
        let mut ccommand = Command::new("pwsh");
        ccommand.args(["-C", "Start-Process", ent_agent_bin.as_ref()]);
        ccommand
    } else {
        Command::new(ent_agent_bin.as_ref())
    };
    command
        .arg(seconds)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
}

fn set_identity(id: &[u8]) -> anyhow::Result<()> {
    let string = String::from_utf8(id.to_vec())?;
    let result = entrust_agent::client::set_age_identity(string, env::var("ENT_AGENT_PIN").ok());
    match result {
        Err(e) if e.kind() == NO_AGENT_ERROR_KIND => {
            thread::sleep(Duration::from_millis(100));
            entrust_agent::client::set_age_identity(
                String::from_utf8(id.to_vec())?,
                env::var("ENT_AGENT_PIN").ok(),
            )
        }
        result => result,
    }?;
    Ok(())
}

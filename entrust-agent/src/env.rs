use std::borrow::Cow;
use std::env::VarError;

const ENT_AGENT_BIN: &str = "ENT_AGENT_BIN";
const ENT_AGENT_PIN: &str = "ENT_AGENT_PIN";
const ENT_AGENT_SECONDS: &str = "ENT_AGENT_SECONDS";
pub const ENT_AGENT_SOCKET_NAME: &str = "ENT_AGENT_SOCKET_NAME";

pub fn agent_bin() -> Cow<'static, str> {
    std::env::var(ENT_AGENT_BIN)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("ent-agent"))
}

pub fn agent_pin() -> Result<String, VarError> {
    std::env::var(ENT_AGENT_PIN)
}

pub fn agent_seconds() -> Cow<'static, str> {
    std::env::var(ENT_AGENT_SECONDS)
        .ok()
        .and_then(|v| v.parse::<usize>().ok().map(|_| Cow::Owned(v)))
        .unwrap_or(Cow::Borrowed("600"))
}

pub fn agent_socket_name() -> Cow<'static, str> {
    std::env::var(ENT_AGENT_SOCKET_NAME)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("entrust-agent.sock"))
}

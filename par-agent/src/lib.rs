use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::io::{BufRead, Write};
use std::sync::LazyLock;
use std::{env, io};

pub mod client;
pub mod server;

/// ASCII 'End of Transmission'
const EOT: u8 = 4;

fn send_serialized<R: Serialize, C: Write>(request: &R, con: &mut C) -> io::Result<()> {
    con.write_all(
        bincode::serialize(request)
            .map_err(io::Error::other)?
            .as_slice(),
    )?;
    con.write_all([EOT].as_ref())
}

fn read_deserialized<R: DeserializeOwned, C: BufRead>(con: &mut C) -> io::Result<R> {
    let mut buf = Vec::with_capacity(32);
    con.read_until(EOT, &mut buf)?;
    buf.pop();
    bincode::deserialize_from(buf.as_slice()).map_err(io::Error::other)
}

static SOCKET_NAME: LazyLock<Cow<str>> = LazyLock::new(|| {
    env::var("PAR_AGENT_SOCKET_NAME")
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("par-agent.sock"))
});

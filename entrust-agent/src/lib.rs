use bincode::{Decode, Encode, config::standard};
use std::borrow::Cow;
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::sync::LazyLock;

pub mod client;
pub mod env;
pub mod server;

#[cfg(windows)]
pub const NO_AGENT_ERROR_KIND: ErrorKind = ErrorKind::NotFound;
#[cfg(not(windows))]
pub const NO_AGENT_ERROR_KIND: ErrorKind = ErrorKind::ConnectionRefused;

/// ASCII 'End of Transmission'
const EOT: u8 = 4;

fn send_serialized<R: Encode, C: Write>(request: &R, con: &mut C) -> io::Result<()> {
    con.write_all(
        bincode::encode_to_vec(request, standard())
            .map_err(io::Error::other)?
            .as_slice(),
    )?;
    con.write_all([EOT].as_ref())
}

fn read_deserialized<R: Decode<()>, C: BufRead>(con: &mut C) -> io::Result<R> {
    let mut buf = Vec::with_capacity(32);
    con.read_until(EOT, &mut buf)?;
    buf.pop();
    bincode::decode_from_slice(buf.as_slice(), standard())
        .map(|(decoded, _)| decoded)
        .map_err(io::Error::other)
}

static SOCKET_NAME: LazyLock<Cow<str>> = LazyLock::new(|| env::agent_socket_name());

use rkyv::Serialize;
use rkyv::api::high::HighSerializer;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use std::borrow::Cow;
use std::io;
use std::io::{ErrorKind, Write};
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

#[macro_export]
macro_rules! receive {
    ($typ:ty, $con:ident) => {{
        let mut buf = Vec::with_capacity(32);
        std::io::BufRead::read_until(&mut $con, $crate::EOT, &mut buf).and_then(|_| {
            buf.pop();
            paste::paste! {
                let archived =
                rkyv::access::<[<Archived $typ>], rkyv::rancor::Error>(buf.as_slice())
                    .map_err(io::Error::other)?;
                rkyv::deserialize::<$typ, rkyv::rancor::Error>(archived)
                    .map_err(io::Error::other)
            }
        })
    }};
}

fn send<S, C>(request: &S, con: &mut C) -> io::Result<()>
where
    S: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>,
    C: Write,
{
    let vec = rkyv::to_bytes(request).map_err(io::Error::other)?;
    con.write_all(vec.as_slice())?;
    con.write_all([EOT].as_ref())
}

static SOCKET_NAME: LazyLock<Cow<str>> = LazyLock::new(env::agent_socket_name);

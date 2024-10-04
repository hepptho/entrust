use crate::server::{GetAgeIdentityResponse, Request};
use crate::{read_deserialized, send_serialized, SOCKET_NAME};
use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{GenericFilePath, GenericNamespaced, Stream};
use std::io;
use std::io::BufReader;

pub fn set_age_identity(identity: String, pin: Option<String>) -> io::Result<()> {
    let request = Request::SetAgeIdentity { identity, pin };
    let mut con = connect()?;
    send_serialized(&request, con.get_mut())
}

pub fn get_age_identity(pin: Option<String>) -> io::Result<GetAgeIdentityResponse> {
    let request = Request::GetAgeIdentity { pin };
    let mut con = connect()?;
    send_serialized(&request, con.get_mut())?;
    let response: GetAgeIdentityResponse = read_deserialized(&mut con)?;
    Ok(response)
}

pub fn shutdown_server() -> io::Result<()> {
    let mut con = connect()?;
    send_serialized(&Request::Shutdown, con.get_mut())
}

pub fn is_server_running() -> bool {
    get_age_identity(None).is_ok()
}

fn connect() -> io::Result<BufReader<Stream>> {
    let socket_name = if GenericNamespaced::is_supported() {
        SOCKET_NAME.as_ref().to_ns_name::<GenericNamespaced>()?
    } else {
        format!("/tmp/{}", SOCKET_NAME.as_ref()).to_fs_name::<GenericFilePath>()?
    };
    Stream::connect(socket_name).map(BufReader::new)
}

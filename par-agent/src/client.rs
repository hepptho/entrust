use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{GenericFilePath, GenericNamespaced, Stream};
use std::io;
use std::io::{BufRead, BufReader, Write};

pub fn set_age_identity<P: AsRef<str>>(identity: &str, pin: Option<P>) -> io::Result<()> {
    let identity = identity
        .lines()
        .find(|&l| !l.starts_with('#'))
        .ok_or(io::Error::other("Invalid age identity"))?;
    let message = format!(
        "set age\n{identity}\n{}\n",
        pin.as_ref().map(|p| p.as_ref()).unwrap_or("-")
    );
    let mut con = connect()?;
    con.get_mut().write_all(message.as_bytes())
}

pub fn get_age_identity<P: AsRef<str>>(pin: Option<P>) -> io::Result<String> {
    let message = format!(
        "get age\n{}\n",
        pin.as_ref().map(|p| p.as_ref()).unwrap_or("-")
    );
    let mut con = connect()?;
    con.get_mut().write_all(message.as_bytes())?;
    let mut buffer = String::with_capacity(128);
    con.read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn shutdown_server() -> io::Result<()> {
    let mut con = connect()?;
    con.get_mut().write_all(b"shutdown\n")
}

fn connect() -> io::Result<BufReader<Stream>> {
    let socket_name = if GenericNamespaced::is_supported() {
        "par-agent.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/par-agent.sock".to_fs_name::<GenericFilePath>()?
    };
    Stream::connect(socket_name).map(BufReader::new)
}

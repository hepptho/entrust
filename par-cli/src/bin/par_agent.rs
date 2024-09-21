use par_agent::server::ServerEvent;
use par_agent::{client, server};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::{env, io, thread};

fn main() -> io::Result<()> {
    let arg = env::args().nth(1);
    let shutdown = arg.as_ref().is_some_and(|arg| arg.as_str() == "shutdown");
    if shutdown {
        client::shutdown_server()?;
    } else {
        let seconds = arg
            .as_ref()
            .map(|s| s.parse::<u64>())
            .transpose()
            .unwrap_or(Some(600))
            .unwrap_or(600);
        let (sender, receiver) = channel();

        thread::spawn(move || server::run(Some(sender)));
        let mut started = match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(ServerEvent::Started) => Ok(Instant::now()),
            _ => Err(io::Error::other("Server did not start")),
        }?;
        loop {
            match receiver.recv_timeout(Duration::from_secs(seconds) - (Instant::now() - started)) {
                Ok(ServerEvent::Stopped) | Err(_) => {
                    println!("stopping");
                    break;
                }
                Ok(ServerEvent::RequestHandled) => {
                    println!("resetting timeout");
                    started = Instant::now()
                }
                Ok(_) => continue,
            };
        }
    }
    Ok(())
}

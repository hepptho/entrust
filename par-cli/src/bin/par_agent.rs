#![windows_subsystem = "windows"]

use std::time::Duration;
use std::{env, io};

fn main() -> io::Result<()> {
    let arg = env::args().nth(1);
    let shutdown = arg.as_ref().is_some_and(|arg| arg.as_str() == "shutdown");
    if shutdown {
        par_agent::client::shutdown_server()
    } else if !par_agent::client::is_server_running() {
        let seconds = arg
            .as_ref()
            .map(|s| s.parse::<u64>())
            .and_then(|r| r.ok())
            .unwrap_or(600);
        par_agent::server::run_with_idle_timeout(Duration::from_secs(seconds))
    } else {
        Ok(())
    }
}

use par_agent::server;
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| server::run(None));
    thread::sleep(Duration::from_secs(600));
}

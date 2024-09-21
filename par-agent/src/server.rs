mod event;

use interprocess::local_socket::traits::ListenerExt;
use interprocess::local_socket::{
    GenericNamespaced, ListenerNonblockingMode, ListenerOptions, ToNsName,
};
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::mpsc;

use crate::server::event::EventSender;
pub use event::ServerEvent;

#[derive(Debug, Default)]
struct State {
    age_identity: String,
    age_password: String,
}

pub fn run(event_sender: Option<mpsc::Sender<ServerEvent>>) -> io::Result<()> {
    let mut state = State::default();

    let socket_name = "par-agent.sock".to_ns_name::<GenericNamespaced>()?;

    let options = ListenerOptions::new()
        .name(socket_name)
        .nonblocking(ListenerNonblockingMode::Neither);
    let listener = options.create_sync()?;

    let mut buffer = String::with_capacity(128);
    event_sender.send_server_event(ServerEvent::Started)?;
    for result in listener.incoming() {
        let mut con = BufReader::new(result?);

        con.read_line(&mut buffer)?;

        let handle_result = handle_request(buffer.as_str(), &mut state, &mut con)?;
        event_sender.send_server_event(ServerEvent::RequestHandled)?;
        if let HandleResult::Break = handle_result {
            break;
        }

        buffer.clear();
    }
    event_sender.send_server_event(ServerEvent::Stopped)?;
    Ok(())
}

#[derive(Debug, PartialEq)]
enum HandleResult {
    Break,
    Continue,
}

fn handle_request<R: Read + Write>(
    request: &str,
    state: &mut State,
    con: &mut BufReader<R>,
) -> io::Result<HandleResult> {
    match request {
        "set age\n" => {
            state.age_identity.clear();
            con.read_line(&mut state.age_identity)?;
            state.age_password.clear();
            con.read_line(&mut state.age_password)?;
        }
        "get age\n" => {
            let mut supplied_password = String::new();
            con.read_line(&mut supplied_password)?;
            if state.age_identity.is_empty() {
                con.get_mut().write_all(b"\n")?;
            } else if supplied_password == state.age_password {
                con.get_mut().write_all(state.age_identity.as_bytes())?;
            } else {
                con.get_mut().write_all(b"-\n")?;
                return Ok(HandleResult::Break);
            }
        }
        "shutdown\n" => return Ok(HandleResult::Break),
        _ => {}
    }
    Ok(HandleResult::Continue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_handle_get_age_empty() {
        let mut state = State::default();
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result = handle_request("get age\n", &mut state, &mut con).unwrap();
        assert_eq!(HandleResult::Continue, result);
        con.get_mut().set_position(0);
        let mut response = String::new();
        con.read_line(&mut response).unwrap();
        assert_eq!("\n", response.as_str())
    }

    #[test]
    fn test_handle_get_age() {
        let mut state = State {
            age_identity: "id\n".to_string(),
            ..Default::default()
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result = handle_request("get age\n", &mut state, &mut con).unwrap();
        assert_eq!(HandleResult::Continue, result);
        con.get_mut().set_position(0);
        let mut response = String::new();
        con.read_to_string(&mut response).unwrap();
        assert_eq!("id\n", response.as_str())
    }

    #[test]
    fn test_handle_get_age_password() {
        let mut state = State {
            age_identity: "id\n".to_string(),
            age_password: "pass\n".to_string(),
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        con.get_mut().write_all(b"pass\n").unwrap();
        con.get_mut().set_position(0);
        let result = handle_request("get age\n", &mut state, &mut con).unwrap();
        assert_eq!(HandleResult::Continue, result);
        con.get_mut().set_position(5);
        let mut response = String::new();
        con.read_to_string(&mut response).unwrap();
        assert_eq!("id\n", response.as_str())
    }

    #[test]
    fn test_handle_get_age_wrong_password() {
        let mut state = State {
            age_identity: "id\n".to_string(),
            age_password: "pass\n".to_string(),
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        con.get_mut().write_all(b"wrong pass\n").unwrap();
        con.get_mut().set_position(0);
        let result = handle_request("get age\n", &mut state, &mut con).unwrap();
        assert_eq!(HandleResult::Break, result);
        con.get_mut().set_position(11);
        let mut response = String::new();
        con.read_to_string(&mut response).unwrap();
        assert_eq!("-\n", response.as_str())
    }
}

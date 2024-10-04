mod event;
mod request;

use crate::server::event::EventSender;
use crate::server::HandleResult::{Break, Continue};
use crate::{read_deserialized, send_serialized};
pub use event::ServerEvent;
use interprocess::local_socket::traits::ListenerExt;
use interprocess::local_socket::{
    GenericNamespaced, ListenerNonblockingMode, ListenerOptions, ToNsName,
};
pub use request::{GetAgeIdentityResponse, Request, SetAgeIdentityResponse};
use std::io;
use std::io::{BufReader, Read, Write};
use std::sync::mpsc;

#[derive(Debug, Default)]
struct State {
    age_identity: String,
    age_pin: Option<String>,
}

pub fn run(event_sender: Option<mpsc::Sender<ServerEvent>>) -> io::Result<()> {
    let mut state = State::default();

    let socket_name = "par-agent.sock".to_ns_name::<GenericNamespaced>()?;

    let options = ListenerOptions::new()
        .name(socket_name)
        .nonblocking(ListenerNonblockingMode::Neither);
    let listener = options.create_sync()?;

    event_sender.send_server_event(ServerEvent::Started)?;
    for result in listener.incoming() {
        let mut con = BufReader::new(result?);

        let request: Request = read_deserialized(&mut con)?;

        let handle_result = handle_request(request, &mut state, &mut con)?;
        event_sender.send_server_event(ServerEvent::RequestHandled)?;
        if let Break = handle_result {
            break;
        }
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
    request: Request,
    state: &mut State,
    con: &mut BufReader<R>,
) -> io::Result<HandleResult> {
    match request {
        Request::SetAgeIdentity { identity, pin } => {
            state.age_identity = identity;
            state.age_pin = pin;
            Ok(Continue)
        }
        Request::GetAgeIdentity { pin } => {
            let response = if state.age_identity.is_empty() {
                GetAgeIdentityResponse::NotSet
            } else if pin == state.age_pin {
                GetAgeIdentityResponse::Ok {
                    identity: state.age_identity.clone(),
                }
            } else {
                GetAgeIdentityResponse::WrongPin
            };
            send_serialized(&response, con.get_mut())?;
            if let GetAgeIdentityResponse::WrongPin = response {
                Ok(Break)
            } else {
                Ok(Continue)
            }
        }
        Request::Shutdown => Ok(Break),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_serialization() {
        let req = Request::SetAgeIdentity {
            identity: "id".to_string(),
            pin: Some("pin".to_string()),
        };
        let mut con = BufReader::new(Cursor::new(Vec::new()));
        send_serialized(&req, con.get_mut()).unwrap();
        con.get_mut().set_position(0);
        let deserialized = read_deserialized(con.get_mut()).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_handle_get_age_empty() {
        let mut state = State::default();
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result =
            handle_request(Request::GetAgeIdentity { pin: None }, &mut state, &mut con).unwrap();
        assert_eq!(Continue, result);
        con.get_mut().set_position(0);
        let response: GetAgeIdentityResponse = read_deserialized(con.get_mut()).unwrap();
        assert_eq!(GetAgeIdentityResponse::NotSet, response);
    }

    #[test]
    fn test_handle_get_age() {
        let mut state = State {
            age_identity: "id".to_string(),
            ..Default::default()
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result =
            handle_request(Request::GetAgeIdentity { pin: None }, &mut state, &mut con).unwrap();
        assert_eq!(Continue, result);
        con.get_mut().set_position(0);
        let response: GetAgeIdentityResponse = read_deserialized(con.get_mut()).unwrap();
        assert_eq!(
            GetAgeIdentityResponse::Ok {
                identity: "id".to_string()
            },
            response
        );
    }

    #[test]
    fn test_handle_get_age_password() {
        let mut state = State {
            age_identity: "id".to_string(),
            age_pin: Some("pass".to_string()),
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result = handle_request(
            Request::GetAgeIdentity {
                pin: Some("pass".to_string()),
            },
            &mut state,
            &mut con,
        )
        .unwrap();
        assert_eq!(Continue, result);
        con.get_mut().set_position(0);
        let response: GetAgeIdentityResponse = read_deserialized(con.get_mut()).unwrap();
        assert_eq!(
            GetAgeIdentityResponse::Ok {
                identity: "id".to_string()
            },
            response
        );
    }

    #[test]
    fn test_handle_get_age_wrong_password() {
        let mut state = State {
            age_identity: "id".to_string(),
            age_pin: Some("pass".to_string()),
        };
        let mut con: BufReader<_> = BufReader::new(Cursor::new(Vec::new()));
        let result = handle_request(
            Request::GetAgeIdentity {
                pin: Some("wrong pass".to_string()),
            },
            &mut state,
            &mut con,
        )
        .unwrap();
        assert_eq!(Break, result);
        con.get_mut().set_position(0);
        let response: GetAgeIdentityResponse = read_deserialized(con.get_mut()).unwrap();
        assert_eq!(GetAgeIdentityResponse::WrongPin, response)
    }
}

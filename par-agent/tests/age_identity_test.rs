use par_agent::client::{get_age_identity, set_age_identity};
use par_agent::server;
use par_agent::server::{GetAgeIdentityResponse, ServerEvent};
use std::io::ErrorKind;
use std::sync::mpsc;
use std::time::Duration;
use std::{env, thread};

#[test]
fn test_age_identity() {
    env::set_var("PAR_AGENT_SOCKET_NAME", "par_test_age_identity.sock");
    let (event_sender, event_receiver) = mpsc::channel::<ServerEvent>();
    thread::spawn(move || server::run(Some(event_sender)).unwrap());
    let started = event_receiver
        .recv_timeout(Duration::from_millis(100))
        .is_ok_and(|event| event == ServerEvent::Started);
    assert!(started, "server did not start");

    assert_eq!(
        GetAgeIdentityResponse::NotSet,
        get_age_identity(Some("pin".to_string())).unwrap()
    );
    assert_eq!(
        Ok(ServerEvent::RequestHandled),
        event_receiver.recv_timeout(Duration::from_millis(10))
    );

    set_age_identity("some identity".to_string(), Some("pin".to_string())).unwrap();
    assert_eq!(
        Ok(ServerEvent::RequestHandled),
        event_receiver.recv_timeout(Duration::from_millis(10))
    );

    assert_eq!(
        GetAgeIdentityResponse::Ok {
            identity: "some identity".to_string()
        },
        get_age_identity(Some("pin".to_string())).unwrap(),
    );
    assert_eq!(
        Ok(ServerEvent::RequestHandled),
        event_receiver.recv_timeout(Duration::from_millis(10))
    );

    assert_eq!(
        GetAgeIdentityResponse::WrongPin,
        get_age_identity(Some("wrong pin".to_string())).unwrap(),
    );
    assert_eq!(
        Ok(ServerEvent::RequestHandled),
        event_receiver.recv_timeout(Duration::from_millis(10))
    );
    assert_eq!(
        Ok(ServerEvent::Stopped),
        event_receiver.recv_timeout(Duration::from_millis(10))
    );

    assert!(
        get_age_identity(Some("pin".to_string())).is_err_and(|e| e.kind() == ErrorKind::NotFound),
        "server should shut down after receiving wrong pin"
    )
}

use par_agent::client::{get_age_identity, set_age_identity};
use par_agent::server;
use par_agent::server::{GetAgeIdentityResponse, ServerEvent};
use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_age_identity() {
    let (started_send, started_recv) = mpsc::channel::<ServerEvent>();
    thread::spawn(move || server::run(Some(started_send)).unwrap());
    let started = started_recv
        .recv_timeout(Duration::from_secs(1))
        .is_ok_and(|event| event == ServerEvent::Started);
    assert!(started);

    assert_eq!(
        GetAgeIdentityResponse::NotSet,
        get_age_identity(Some("pin".to_string())).unwrap()
    );

    set_age_identity("some identity".to_string(), Some("pin".to_string())).unwrap();

    assert_eq!(
        GetAgeIdentityResponse::Ok {
            identity: "some identity".to_string()
        },
        get_age_identity(Some("pin".to_string())).unwrap(),
    );

    assert_eq!(
        GetAgeIdentityResponse::WrongPin,
        get_age_identity(Some("wrong pin".to_string())).unwrap(),
    );
    assert!(
        get_age_identity(Some("pin".to_string())).is_err_and(|e| e.kind() == ErrorKind::NotFound),
        "server should shut down after receiving wrong pin"
    )
}

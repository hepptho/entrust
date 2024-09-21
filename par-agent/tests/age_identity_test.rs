use par_agent::client::{get_age_identity, set_age_identity};
use par_agent::server;
use par_agent::server::ServerEvent;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_age_identity() {
    let (started_send, started_recv) = mpsc::channel::<ServerEvent>();
    println!("start");
    thread::spawn(move || server::run(Some(started_send)).unwrap());
    let started = started_recv
        .recv_timeout(Duration::from_secs(1))
        .is_ok_and(|event| event == ServerEvent::Started);
    assert!(started);
    println!("first get");
    assert_eq!("\n", get_age_identity(Some("pin")).unwrap().as_str());
    println!("set");
    set_age_identity("some identity", Some("pin")).unwrap();
    println!("second get");
    assert_eq!(
        "some identity\n",
        get_age_identity(Some("pin")).unwrap().as_str()
    );
}

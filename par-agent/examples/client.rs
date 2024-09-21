use par_agent::client::{get_age_identity, set_age_identity};
use std::io;

fn main() -> io::Result<()> {
    let received = get_age_identity(Some("pin"))?;
    println!("Client: Got age identity: {received:?}");
    println!("Client: Setting age identity");
    set_age_identity("some identity", Some("pin"))?;
    println!("Client: Getting age identity");
    let received = get_age_identity(Some("pin"))?;
    println!("Client: Got age identity: {received:?}");
    Ok(())
}

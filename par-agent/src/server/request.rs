use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Request {
    SetAgeIdentity {
        identity: String,
        pin: Option<String>,
    },
    GetAgeIdentity {
        pin: Option<String>,
    },
    Shutdown,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SetAgeIdentityResponse {
    identity: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum GetAgeIdentityResponse {
    Ok { identity: String },
    NotSet,
    WrongPin,
}

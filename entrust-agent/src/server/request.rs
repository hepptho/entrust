use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
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

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct SetAgeIdentityResponse {
    identity: String,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum GetAgeIdentityResponse {
    Ok { identity: String },
    NotSet,
    WrongPin,
}

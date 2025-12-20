use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug)]
pub struct SetAgeIdentityResponse {
    identity: String,
}

#[derive(Archive, Serialize, Deserialize, PartialEq, Debug)]
pub enum GetAgeIdentityResponse {
    Ok { identity: String },
    NotSet,
    WrongPin,
}

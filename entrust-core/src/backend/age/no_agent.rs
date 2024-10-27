use crate::age::identity::read_identity;
use anyhow::anyhow;
use std::sync::OnceLock;

static IDENTITY: OnceLock<anyhow::Result<Vec<u8>>> = OnceLock::new();

pub fn get_identity() -> anyhow::Result<&'static Vec<u8>> {
    IDENTITY
        .get_or_init(read_identity)
        .as_ref()
        .map_err(|err| anyhow!(err))
}

use crate::error::ParResult;
use anyhow::anyhow;
use std::env::VarError;
use std::path::PathBuf;
use std::{env, fs};

const STORE_ENV_VAR: &str = "PAR_STORE";

pub fn get_store() -> ParResult<PathBuf> {
    let ret = match env::var(STORE_ENV_VAR) {
        Ok(store) => {
            let store = PathBuf::from(store);
            if store.exists() && !store.is_dir() {
                Err(anyhow!("{:?} exists and is not a file", store.as_os_str()))
            } else {
                fs::create_dir_all(&store)?;
                Ok(store)
            }
        }
        Err(VarError::NotPresent) => Err(anyhow!("\"{STORE_ENV_VAR}\" is not set")),
        Err(VarError::NotUnicode(_)) => Err(anyhow!("\"{STORE_ENV_VAR}\" contains invalid utf-8")),
    }?;
    Ok(ret)
}

use crate::dialog::{read_key_interactive, select_existing_key};
use std::io;
use std::path::Path;

pub trait Key: Into<Option<String>> {
    fn unwrap_or_read(self, prompt: &'static str) -> io::Result<String> {
        match self.into() {
            Some(key) => Ok(key),
            None => read_key_interactive(prompt),
        }
    }
    fn unwrap_or_select_existing(self, store: &Path) -> anyhow::Result<String> {
        match self.into() {
            Some(key) => Ok(key),
            None => select_existing_key(store),
        }
    }
}

impl Key for Option<String> {}

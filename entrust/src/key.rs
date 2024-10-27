use crate::dialog::{read_new_key_interactive, select_existing_key};
use std::path::Path;

pub trait Key: Into<Option<String>> {
    fn unwrap_or_read_new(self, prompt: &'static str, store: &Path) -> anyhow::Result<String> {
        match self.into() {
            Some(key) => Ok(key),
            None => read_new_key_interactive(prompt, store),
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

pub(crate) mod age;
pub(crate) mod gpg;

use anyhow::anyhow;
use clap::ValueEnum;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Backend {
    Age,
    Gpg,
}

impl Backend {
    pub fn encrypt(
        &self,
        mut content: impl Read,
        store: &Path,
        out_path: &Path,
    ) -> anyhow::Result<()> {
        match self {
            Backend::Age => {
                age::encrypt(&mut content, &self.recipient(store)?, out_path)?;
            }
            Backend::Gpg => {
                gpg::encrypt(&mut content, &self.recipient(store)?, out_path)?;
            }
        }
        Ok(())
    }

    pub fn decrypt(path: &Path) -> anyhow::Result<String> {
        if is_age_encrypted(path)? {
            age::decrypt(path)
        } else {
            gpg::decrypt(path)
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Backend::Age => "age",
            Backend::Gpg => "gpg",
        }
    }

    pub fn recipient_file_name(&self) -> &'static str {
        match self {
            Backend::Age => age::RECIPIENT_FILE_NAME,
            Backend::Gpg => gpg::RECIPIENT_FILE_NAME,
        }
    }

    pub fn needs_init(self, store: &Path) -> Option<Backend> {
        if store.join(self.recipient_file_name()).exists() {
            None
        } else {
            Some(self)
        }
    }

    fn recipient(&self, dir: &Path) -> anyhow::Result<String> {
        let recipient_file = dir.join(self.recipient_file_name());
        read_first_line(&recipient_file)
    }
}

fn is_age_encrypted(path: &Path) -> anyhow::Result<bool> {
    let first_line = read_first_line(path)?;
    Ok(
        first_line.contains("BEGIN AGE ENCRYPTED FILE")
            || first_line.contains("age-encryption.org"),
    )
}

fn read_first_line(path: &Path) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let first_line = BufReader::new(file)
        .lines()
        .next()
        .ok_or(anyhow!("{path:?} is empty"))??;
    Ok(first_line)
}

pub(crate) mod age;
pub(crate) mod gpg;

use crate::error::ParResult;
use crate::theme::INQUIRE_RENDER_CONFIG;
use anyhow::anyhow;
use clap::ValueEnum;
use inquire::Text;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Backend {
    Age,
    Gpg,
}

impl Backend {
    pub fn encrypt(&self, mut content: impl Read, store: &Path, out_path: &Path) -> ParResult<()> {
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

    pub fn decrypt(path: &Path) -> ParResult<String> {
        if is_age_encrypted(path)? {
            age::decrypt(path)
        } else {
            gpg::decrypt(path)
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Backend::Age => "age",
            Backend::Gpg => "gpg",
        }
    }

    fn recipient_file_name(&self) -> &'static str {
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

    pub fn create_recipient_file_if_not_present(&self, store: &Path) -> ParResult<()> {
        let file = store.join(self.recipient_file_name());
        if file.exists() {
            return Ok(());
        }
        let recipient: String = Text::new(
            format!(
                "{} recipient for which the file should be created ❯",
                self.display_name()
            )
            .as_str(),
        )
        .with_render_config(*INQUIRE_RENDER_CONFIG)
        .prompt()?;
        fs::write(file, recipient.as_bytes())?;
        Ok(())
    }

    fn recipient(&self, dir: &Path) -> ParResult<String> {
        let recipient_file = dir.join(self.recipient_file_name());
        read_first_line(&recipient_file)
    }
}

fn is_age_encrypted(path: &Path) -> ParResult<bool> {
    let first_line = read_first_line(path)?;
    Ok(
        first_line.contains("BEGIN AGE ENCRYPTED FILE")
            || first_line.contains("age-encryption.org"),
    )
}

fn read_first_line(path: &Path) -> ParResult<String> {
    let file = File::open(path)?;
    let first_line = BufReader::new(file)
        .lines()
        .next()
        .ok_or(anyhow!("{path:?} is empty"))??;
    Ok(first_line)
}

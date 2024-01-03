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

#[derive(ValueEnum, Clone, Debug)]
pub enum Backend {
    Age,
    Gpg,
}

impl Backend {
    pub fn encrypt(&self, mut content: impl Read, store: &Path, out_path: &Path) -> ParResult<()> {
        match self {
            Backend::Age => {
                self.create_recipient_file_if_not_present(store, "age")?;
                age::encrypt(&mut content, &self.recipient(store)?, out_path)?;
            }
            Backend::Gpg => {
                self.create_recipient_file_if_not_present(store, "gpg")?;
                gpg::encrypt(&mut content, &self.recipient(store)?, out_path)?;
            }
        }
        Ok(())
    }

    pub fn decrypt(path: &Path) -> ParResult<String> {
        let first_line = read_first_line(path)?;
        if first_line.contains("BEGIN AGE ENCRYPTED FILE")
            || first_line.contains("age-encryption.org")
        {
            age::decrypt(path)
        } else {
            gpg::decrypt(path)
        }
    }

    fn recipient_file_name(&self) -> &'static str {
        match self {
            Backend::Age => age::RECIPIENT_FILE_NAME,
            Backend::Gpg => gpg::RECIPIENT_FILE_NAME,
        }
    }

    fn create_recipient_file_if_not_present(
        &self,
        store: &Path,
        display_name: &str,
    ) -> ParResult<()> {
        let file = store.join(self.recipient_file_name());
        if file.exists() {
            return Ok(());
        }
        let recipient: String = Text::new(
            format!("{display_name} recipient for which the file should be created ❯").as_str(),
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

fn read_first_line(path: &Path) -> ParResult<String> {
    let file = File::open(path)?;
    let first_line = BufReader::new(file)
        .lines()
        .next()
        .ok_or(anyhow!("{path:?} is empty"))??;
    Ok(first_line)
}

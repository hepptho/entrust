pub(crate) mod age;
pub(crate) mod gpg;

use crate::backend::age::Age;
use crate::backend::gpg::Gpg;
use crate::command::BackendOption;
use crate::error::ParResult;
use crate::git;
use crate::theme::DIALOGUER_THEME;
use anyhow::anyhow;
use dialoguer::Input;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub trait Backend {
    fn encrypt(content: &mut impl Read, recipient: &str, out_path: &Path) -> ParResult<()>;

    fn decrypt(path: &Path) -> ParResult<String>;

    fn recipient_file_name() -> &'static str;

    fn create_recipient_file_if_not_present(dir: &Path, display_name: &str) -> ParResult<()> {
        let file = dir.join(Self::recipient_file_name());
        if file.exists() {
            return Ok(());
        }
        let recipient: String = Input::with_theme(&*DIALOGUER_THEME)
            .with_prompt(format!(
                "Please enter the {display_name} recipient for which the file should be created"
            ))
            .interact_text()?;
        fs::write(file, recipient.as_bytes())?;
        Ok(())
    }

    fn recipient(dir: &Path) -> ParResult<String> {
        let recipient_file = dir.join(Self::recipient_file_name());
        read_first_line(&recipient_file)
    }
}

pub fn decrypt_file(path: &Path) -> ParResult<String> {
    let first_line = read_first_line(path)?;
    if first_line.contains("BEGIN AGE ENCRYPTED FILE") {
        Age::decrypt(path)
    } else {
        Gpg::decrypt(path)
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

pub fn encrypt_with_backend(
    backend: &BackendOption,
    mut content: impl Read,
    home: &Path,
    out_path: &Path,
) -> ParResult<()> {
    let name = match backend {
        BackendOption::Age => {
            Age::create_recipient_file_if_not_present(home, "age")?;
            Age::encrypt(&mut content, &Age::recipient(home)?, out_path)?;
            Age::recipient_file_name()
        }
        BackendOption::Gpg => {
            Gpg::create_recipient_file_if_not_present(home, "gpg")?;
            Gpg::encrypt(&mut content, &Gpg::recipient(home)?, out_path)?;
            Gpg::recipient_file_name()
        }
    };
    git::add(home, name)?;
    Ok(())
}

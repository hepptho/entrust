use crate::error::ParResult;
use anyhow::anyhow;
use log::debug;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn init(home: &Path) -> ParResult<()> {
    run_command(git().args(["init", "--initial-branch", "main"]), home, true)?;
    run_command(git().args(["add", "*"]), home, true)?;
    run_command(
        git().args(["commit", "--message", "initialize rp password store repo"]),
        home,
        true,
    )?;
    Ok(())
}

pub fn add(home: &Path, key: &str) -> ParResult<()> {
    debug!("git add(home: {home:?}, key: {key})");
    if has_repository(home) {
        run_command(git().arg("add").arg(home.join(key).as_os_str()), home, true)?;
        run_command(git().args(["commit", "--message", "add", key]), home, true)?;
    }
    Ok(())
}

pub fn edit(home: &Path, key: &str) -> ParResult<()> {
    if has_repository(home) && is_file_tracked(home, key) {
        run_command(git().arg("add").arg(home.join(key).as_os_str()), home, true)?;
        run_command(
            git().args(["commit", "--message", &format!("edit {}", key)]),
            home,
            true,
        )?;
    }
    Ok(())
}

pub fn r#move(home: &Path, from_key: &str, to_key: &str) -> ParResult<bool> {
    if has_repository(home) && is_file_tracked(home, from_key) {
        run_command(
            git()
                .arg("mv")
                .arg(home.join(from_key).as_os_str())
                .arg(home.join(to_key).as_os_str()),
            home,
            true,
        )?;
        run_command(
            git().args([
                "commit",
                "--message",
                &format!("move {} to {}", from_key, to_key),
            ]),
            home,
            true,
        )?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn remove(home: &Path, key: &str) -> ParResult<()> {
    if has_repository(home) && is_file_tracked(home, key) {
        run_command(git().arg("rm").arg(home.join(key).as_os_str()), home, true)?;
        run_command(
            Command::new("git").args(["commit", "--message", &format!("remove {}", key)]),
            home,
            true,
        )?;
    }
    Ok(())
}

fn run_command(command: &mut Command, home: &Path, inherit_io: bool) -> ParResult<()> {
    let stdio = || match inherit_io {
        true => Stdio::inherit(),
        false => Stdio::null(),
    };
    let result = command
        .current_dir(home)
        .stdin(stdio())
        .stdout(stdio())
        .stderr(stdio())
        .status();
    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(anyhow!(
            "git failed with status {}",
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or("unknown".to_string())
        )
        .into()),
        Err(err) => Err(err.into()),
    }
}

fn has_repository(home: &Path) -> bool {
    home.join(".git").is_dir()
}

fn is_file_tracked(home: &Path, key: &str) -> bool {
    run_command(
        git()
            .arg("ls-files")
            .arg("--error-unmatch")
            .arg(home.join(key).as_os_str()),
        home,
        false,
    )
    .is_ok()
}

fn git() -> Command {
    Command::new("git")
}

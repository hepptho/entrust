use crate::error::ParResult;
use anyhow::anyhow;
use log::debug;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn init(store: &Path) -> ParResult<()> {
    run_command(
        git().args(["init", "--initial-branch", "main"]),
        store,
        true,
    )?;
    run_command(git().args(["add", "*"]), store, true)?;
    run_command(
        git().args(["commit", "--message", "initialize rp password store repo"]),
        store,
        true,
    )?;
    Ok(())
}

pub fn add(store: &Path, key: &str) -> ParResult<()> {
    debug!("git add(store: {store:?}, key: {key})");
    if has_repository(store) {
        run_command(
            git().arg("add").arg(store.join(key).as_os_str()),
            store,
            true,
        )?;
        run_command(
            git().args(["commit", "--message", &format!("add {key}")]),
            store,
            true,
        )?;
    }
    Ok(())
}

pub fn edit(store: &Path, key: &str) -> ParResult<()> {
    if has_repository(store) && is_file_tracked(store, key) {
        run_command(
            git().arg("add").arg(store.join(key).as_os_str()),
            store,
            true,
        )?;
        run_command(
            git().args(["commit", "--message", &format!("edit {}", key)]),
            store,
            true,
        )?;
    }
    Ok(())
}

pub fn r#move(store: &Path, from_key: &str, to_key: &str) -> ParResult<bool> {
    if has_repository(store) && is_file_tracked(store, from_key) {
        run_command(
            git()
                .arg("mv")
                .arg(store.join(from_key).as_os_str())
                .arg(store.join(to_key).as_os_str()),
            store,
            true,
        )?;
        run_command(
            git().args([
                "commit",
                "--message",
                &format!("move {} to {}", from_key, to_key),
            ]),
            store,
            true,
        )?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn remove(store: &Path, key: &str) -> ParResult<()> {
    if has_repository(store) && is_file_tracked(store, key) {
        run_command(
            git().arg("rm").arg(store.join(key).as_os_str()),
            store,
            true,
        )?;
        run_command(
            Command::new("git").args(["commit", "--message", &format!("remove {}", key)]),
            store,
            true,
        )?;
    }
    Ok(())
}

fn run_command(command: &mut Command, store: &Path, inherit_io: bool) -> ParResult<()> {
    let stdio = || match inherit_io {
        true => Stdio::inherit(),
        false => Stdio::null(),
    };
    let result = command
        .current_dir(store)
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
        )),
        Err(err) => Err(err.into()),
    }
}

fn has_repository(store: &Path) -> bool {
    store.join(".git").is_dir()
}

fn is_file_tracked(store: &Path, key: &str) -> bool {
    run_command(
        git()
            .arg("ls-files")
            .arg("--error-unmatch")
            .arg(store.join(key).as_os_str()),
        store,
        false,
    )
    .is_ok()
}

fn git() -> Command {
    Command::new("git")
}

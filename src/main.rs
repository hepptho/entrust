use anyhow::anyhow;
use clap::{CommandFactory, Parser};
use log::debug;
use par::command::{
    add, clear_clipboard, completions, edit, generate, get, r#move, remove, Par, ParSubcommands,
};
use par::error::ParResult;
use par::tree::print_tree;
use std::env::VarError;
use std::path::PathBuf;
use std::{env, fs};

const STORE_ENV_VAR: &str = "PAR_STORE";

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let par = Par::parse();

    debug!("{par:#?}");

    let store = get_store()?;
    debug!("store: {store:?}");

    match par.command {
        Some(ParSubcommands::Add(args)) => add::run(store, args),
        Some(ParSubcommands::ClearClipboard(args)) => clear_clipboard::run(args),
        Some(ParSubcommands::Edit(args)) => edit::run(store, args),
        Some(ParSubcommands::Generate(args)) => generate::run(store, args),
        Some(ParSubcommands::Get(args)) => get::run(store, args),
        Some(ParSubcommands::Move(args)) => r#move::run(store, args),
        Some(ParSubcommands::Remove(args)) => remove::run(store, args),
        Some(ParSubcommands::Completions(args)) => {
            completions::run(args);
            Ok(())
        }
        None => {
            Par::command().print_help()?;
            print_tree(&store)?;
            Ok(())
        }
    }
}

fn get_store() -> ParResult<PathBuf> {
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

use clap::Args;
use std::path::PathBuf;
use std::process::Command;

pub(super) const ABOUT: &str = "Run git commands in the password store";

#[derive(Args, Debug)]
pub struct GitArgs {
    /// The arguments to pass to git
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

pub fn run(store: PathBuf, args: GitArgs) -> anyhow::Result<()> {
    Command::new("git")
        .current_dir(store)
        .args(args.args.as_slice())
        .spawn()?
        .wait()?;
    Ok(())
}

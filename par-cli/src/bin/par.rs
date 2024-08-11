#[cfg(feature = "tracing")]
#[path = "../tracing.rs"]
mod tracing;

#[cfg(feature = "tracing")]
use crate::tracing::init_tracing;
use clap::Parser;
use par_cli::alias::apply_aliases;
use par_cli::command;
use par_cli::command::ParArgs;
use std::env;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    init_tracing()?;
    let mut args: Vec<_> = env::args().collect();
    apply_aliases(&mut args);
    command::run(ParArgs::parse_from(args))
}

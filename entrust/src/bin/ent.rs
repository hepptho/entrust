#[cfg(feature = "tracing")]
#[path = "../tracing.rs"]
mod tracing;

use clap::Parser;
use entrust::alias::apply_aliases;
use entrust::command;
use entrust::command::EntArgs;
use std::env;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    tracing::init_tracing()?;
    let mut args: Vec<_> = env::args().collect();
    apply_aliases(&mut args);
    command::run(EntArgs::parse_from(args))
}

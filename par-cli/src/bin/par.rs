use clap::Parser;
use par_cli::alias::apply_aliases;
use par_cli::command;
use par_cli::command::ParArgs;
use std::env;
#[cfg(feature = "tracing")]
use {std::env, std::fs::OpenOptions, std::sync::Arc, tracing_subscriber::EnvFilter};

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    init_tracing()?;
    let mut args: Vec<_> = env::args().collect();
    apply_aliases(&mut args);
    command::run(ParArgs::parse_from(args))
}

#[cfg(feature = "tracing")]
fn init_tracing() -> anyhow::Result<()> {
    if let Ok(log_file) = env::var("PAR_LOG_FILE") {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(Arc::new(log_file))
            .with_ansi(false)
            .init();
    }
    Ok(())
}

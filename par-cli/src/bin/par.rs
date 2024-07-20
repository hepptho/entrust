use clap::Parser;
use par_cli::command;
use par_cli::command::ParArgs;
#[cfg(feature = "tracing")]
use {std::env, std::fs::OpenOptions, std::sync::Arc, tracing_subscriber::EnvFilter};

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tracing")]
    init_tracing()?;
    command::run(ParArgs::parse())
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

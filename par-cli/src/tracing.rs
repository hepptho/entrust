use std::env;
use std::fs::OpenOptions;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

pub fn init_tracing() -> anyhow::Result<()> {
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

use clap::Parser;
use par_cli::command;
use par_cli::command::ParArgs;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    command::run(ParArgs::parse())
}

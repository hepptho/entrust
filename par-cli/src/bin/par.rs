use clap::Parser;
use par_cli::command;
use par_cli::command::ParArgs;

fn main() -> anyhow::Result<()> {
    command::run(ParArgs::parse())
}

use clap::Parser;
use par::command;
use par::command::ParArgs;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    command::run(ParArgs::parse())
}

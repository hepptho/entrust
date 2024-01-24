use par::command;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    command::run()
}

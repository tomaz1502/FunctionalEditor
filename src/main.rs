use std::env;

mod mods;

use mods::config::Config;
use mods::state::State;
use mods::interface;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();

    let config = Config::new(&args, height, width)?;

    let mut state = State::create(config);

    // termion::async_stdin();

    interface::run(&mut state);
    Ok(())
}

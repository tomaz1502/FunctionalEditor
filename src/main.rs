use std::env;
use std::io::stdin;

mod mods;

use mods::config;
use mods::state::State;
use mods::term;

fn main() -> Result<(), &'static str> {
    let stdin = stdin();

    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();

    let config = config::Config::new(&args, height, width)?;
    let mut state = State::new(2, config.left_most(), config);

    term::start_term(&mut state);
    // termion::async_stdin();

    term::run(stdin, &mut state);
    Ok(())
}

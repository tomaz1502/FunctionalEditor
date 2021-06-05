use std::env;
use std::io::stdin;

mod mods;

use mods::config::Config;
use mods::state::State;
use mods::term;

fn main() -> Result<(), &'static str> {
    let stdin = stdin();

    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();

    let config = Config::new(&args, height, width)?;
    let mut state = State::new(0, config.min_col(), config);

    term::start_term(&mut state);
    // termion::async_stdin();

    term::run(stdin, &mut state);
    Ok(())
}

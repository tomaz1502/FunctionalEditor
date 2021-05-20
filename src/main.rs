use std::env;
use std::io::stdin;
use std::process;

mod mods;

use mods::config;
use mods::state::State;
use mods::term;

fn main() {

    let stdin = stdin();

    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();

    let config = config::Config::new(&args, height, width).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut state = State::new(2, config.left_most(), config);

    term::start_term(&mut state);
    // termion::async_stdin();

    term::run(stdin, &mut state);
}

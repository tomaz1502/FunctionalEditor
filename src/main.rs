use termion::input::TermRead;
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

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let (width, height) = termion::terminal_size().unwrap();

    let mut state = State::new(2, 3, height as usize, width as usize);

    term::start_term(&mut state, &config);
    // termion::async_stdin();

    for key in stdin.keys() {
        term::interpret_key(key.unwrap(), &mut state);
    }

    term::die(&mut state);
}

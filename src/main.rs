use termion::input::TermRead;
use std::io::stdin;

mod mods;

use mods::state::State;
use mods::term;


fn main() {
    let stdin = stdin();

    let (width, height) = termion::terminal_size().unwrap();

    let mut state = State::new(2, 3, height as usize, width as usize);

    term::start_term(&mut state);
    // termion::async_stdin();

    for key in stdin.keys() {
        term::interpret_key(key.unwrap(), &mut state);
    }

    term::die(&mut state);
}

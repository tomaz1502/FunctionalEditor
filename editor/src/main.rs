#![allow(unused_imports)]
#![allow(dead_code)]

use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

mod mods;

use mods::state::State;
use mods::term;


fn main() {
    let stdin = stdin();

    let (width, height) = termion::terminal_size().unwrap();

    let mut state = State { row: 2,
                            col: 3,
                            max_row: height,
                            max_col: width,
                            stdout: stdout().into_raw_mode().unwrap()};

    term::start_term(&mut state);
    termion::async_stdin();

    for key in stdin.keys() {
        term::interpret_key(key.unwrap(), &mut state);
    }

    term::die(&mut state);
}

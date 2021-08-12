use std::io;

use termion::event::Key;
use termion::input::TermRead;

use super::state::State;

pub fn run(mut state: &mut State) {
    for key in io::stdin().keys() {
        interpret_key(key.unwrap(), &mut state);
    }
}

fn interpret_key(key: Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => state.break_line(),
        Key::Char(c)      => state.place_char(c),
        Key::Backspace    => state.run_backspace(),
        Key::Left         => state.move_cursor(0, -1),
        Key::Right        => state.move_cursor(0, 1),
        Key::Up           => state.move_cursor(-1, 0),
        Key::Down         => state.move_cursor(1, 0),
        Key::Alt('s')     => state.save_file(),
        Key::Alt('q')     => state.die(),
        _                 => (),
    }
}

pub fn run_prompt(msg: &str, state: &mut State) -> String {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut pointer: usize = 0;
    state.set_message(msg);
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\x0A') => { state.set_message(""); break; }
            Key::Char(c)      => { buffer.insert(pointer, c); pointer += 1; }
            Key::Left         => { if pointer > 0            { pointer -= 1; } },
            Key::Right        => { if pointer < buffer.len() { pointer += 1; } },
            Key::Backspace    => { buffer.remove(pointer); pointer -= 1; }
            _ => ()
        }
        state.set_message(&format!("{}{}", msg, buffer)[..]);
    }
    buffer
}

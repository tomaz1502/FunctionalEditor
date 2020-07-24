use std::io::Write;
use std::iter::FromIterator;
use std::process;

use termion::color;
use termion::event::Key;

use super::state::State;

pub fn die(state: &mut State) {
    let goodbye_message = "Good Bye!";
    let first_line_col = (state.config.max_col() as usize - goodbye_message.len()) / 2;

    write!(
        state.stdout,
        "{}{}{}{}",
        termion::cursor::Show,
        termion::clear::All,
        termion::cursor::Goto(first_line_col as u16, 1),
        goodbye_message)
        .unwrap();

    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();
    process::exit(0);
}

pub fn start_term(state: &mut State) {
    let welcome_message = "Welcome to the Functional Editor";
    let first_line_col = (state.config.max_col() as usize - welcome_message.len()) / 2;

    write!(
        state.stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(first_line_col as u16, 1),
        termion::cursor::Show,
        welcome_message
    )
    .unwrap();

    write!(
        state.stdout,
        "{}{}{}{}",
        color::Fg(color::Yellow),
        termion::cursor::Goto(1, 2),
        1,
        color::Fg(color::Reset)
    )
    .unwrap();

    for row in 3..=state.config.max_row() as u16 {
        write!(state.stdout, "{}", termion::cursor::Goto(1, row)).unwrap();
        print!("~")
    }

    state.go_to(state.row(), state.col());

    if let Some(input_text) = &state.config.text {
        let input_text_clone = input_text.clone();
        draw_file(state, input_text_clone);
    }

    state.stdout.flush().unwrap();
}

fn draw_file(state: &mut State, input_text: String) {
    let mut buffer: Vec<char> = Vec::new();
    for ch in input_text.chars() {
        if ch == '\n' || buffer.len() >= state.config.max_col() as usize - 1 {
            let line = String::from_iter(buffer.iter());
            write!(state.stdout, "{}", line).unwrap();

            state.add_row(Some(buffer));
            state.move_cursor(1, 0);

            buffer = Vec::new();
        }

        if ch != '\n' {
            buffer.push(ch);
        }
    }
}

fn interpret_char(c: char, state: &mut State) {
    print!("{}", c);

    if state.row_length(state.row()) > state.config.max_col() - 2 {
        state.current_row().pop();
    }

    state.current_row().push(c);
    state.move_cursor(0, 1);
}

fn interpret_enter(state: &mut State) {
    state.add_row(None);
    state.move_cursor(1, 0);
}

pub fn interpret_key(key: Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => interpret_enter(state),
        Key::Char(c) => interpret_char(c, state),
        Key::Left => state.move_cursor(0, -1),
        Key::Right => state.move_cursor(0, 1),
        Key::Up => state.move_cursor(-1, 0),
        Key::Down => state.move_cursor(1, 0),
        Key::PageUp => state.move_cursor(2 - state.row() as i16, 0),
        Key::PageDown => state.move_cursor(state.config.max_row() as i16, 0),
        Key::Backspace => (),
        Key::Alt('q') => die(state),
        _ => (),
    }
}

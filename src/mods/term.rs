use std::process;
use std::io::Write;
use std::iter::FromIterator;

use termion::event::Key;
use termion::color;

use super::state::State;
use super::state::Row;
use super::config;

pub fn die(state: &mut State) {
    let goodbye_message = "Good Bye!";
    let first_line_col = (state.max_col - goodbye_message.len()) / 2;

    write!(state.stdout, "{}{}{}{}",
                           termion::cursor::Show,
                           termion::clear::All,
                           termion::cursor::Goto(first_line_col as u16, 1),
                           goodbye_message)
                           .unwrap();

    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();
    process::exit(0);
}

pub fn start_term(state: &mut State, config: &config::Config) {
    let welcome_message = "Welcome to the Functional Editor";
    let first_line_col = (state.max_col - welcome_message.len()) / 2;

    write!(state.stdout, "{}{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(first_line_col as u16, 1),
           termion::cursor::Show,
           welcome_message)
           .unwrap();

    write!(state.stdout, "{}{}{}{}",
                         color::Fg(color::Yellow),
                         termion::cursor::Goto(1,2),
                         1,
                         color::Fg(color::Reset)).
                         unwrap();

    for j in 3 .. state.max_row as u16 {
        write!(state.stdout, "{}", termion::cursor::Goto(1, j)).unwrap();
        print!("~")
    }

    draw_file(state, &config.text);
}

fn draw_file(state: &mut State, file_text: &Option<String>) {
    state.go_to(state.row(), state.col());

    match &file_text {
        Some(input_text) => {
            let mut buffer: Vec<char> = Vec::new();
            for ch in input_text.chars() {
                if ch == '\n' || buffer.len() >= state.max_col - 1 {
                    let line = String::from_iter(buffer.iter());
                    write!(state.stdout, "{}", line).unwrap();
                    add_row(state, Some(buffer));
                    state.move_cursor(1, 0);
                    buffer = Vec::new();
                }
                if ch != '\n' {
                    buffer.push(ch);
                }
            }
            state.stdout.flush().unwrap();
        }
        None    => (),
    }   
}

fn interpret_char(c: char, state: &mut State) {
    print!("{}", c);

    if state.current_row().length() > state.max_col - 2 {
        state.current_row().pop();
    }

    state.current_row().push(c);
    state.move_cursor(0, 1);
}

fn add_row(state: &mut State, to_add: Option<Vec<char>>) {
    match to_add {
        Some(row) => state.rows.push(Row::from_source(row)),
        None      => state.rows.push(Row::new()),
    }
    state.active_rows += 1;
    write!(state.stdout, "{}{}{}{}",
                         color::Fg(color::Yellow),
                         termion::cursor::Goto(1, state.active_rows as u16),
                         state.active_rows - 1,
                         color::Fg(color::Reset)).
                         unwrap();
}

fn interpret_enter(state: &mut State) {
    add_row(state, None);
    state.move_cursor(1, 0);
}

pub fn interpret_key(key : Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => interpret_enter(state),
        Key::Char(c)      => interpret_char(c, state),
        Key::Left         => state.move_cursor(0, -1),
        Key::Right        => state.move_cursor(0,  1),
        Key::Up           => state.move_cursor(-1, 0),
        Key::Down         => state.move_cursor(1,  0),
        Key::PageUp       => state.move_cursor(2 - state.row() as i8, 0),
        Key::PageDown     => state.move_cursor(state.max_row as i8, 0),
        Key::Backspace    => (),
        Key::Alt('q')     => die(state),
        _                 => (),
    }
}

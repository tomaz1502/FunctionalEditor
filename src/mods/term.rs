use std::cmp;
use std::io::Stdin;
use std::io::Write;
use std::iter::FromIterator;
use std::process;

use termion::event::Key;
use termion::input::TermRead;

use super::state::State;

/* Turn the terminal back from Raw mode and ends the program */
pub fn die(state: &mut State) {
    let goodbye_message: &str = "Good Bye!";
    let first_line_col: usize = (state.config.max_col() as usize - goodbye_message.len()) / 2;

    write!(
        state.stdout,
        "{}{}{}{}",
        termion::cursor::Show,
        termion::clear::All,
        termion::cursor::Goto(first_line_col as u16, 1),
        goodbye_message
    )
    .unwrap();

    write!(state.stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();

    process::exit(0);
}

/* Write the welcome message in the terminal, as well as all the '~'.
 * Also, it handles input files given from the command line. */
pub fn start_term(state: &mut State) {
    write!(
        state.stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Show,
    )
    .unwrap();

    for row in 2..=state.config.max_row() as u16 {
        write!(state.stdout, "{}~", termion::cursor::Goto(1, row)).unwrap();
    }
    state.add_row(None);
    eprintln!("{0} {1}", state.row(), state.col());
    state.go_to(state.row(), state.col());

    if let Some(input_text) = &state.config.text {
        let input_text_clone = input_text.clone();
        draw_file(state, input_text_clone);
    }
    state.stdout.flush().unwrap();
}

/* Given the text from a external file (input_text), write it's contexts on the screen
 * and add the necessary rows in state, with its contents. This function was only tested
 * when called by start_term */
fn draw_file(state: &mut State, input_text: String) {
    let mut buffer: Vec<char> = Vec::new();
    for (i, ch) in input_text.chars().enumerate() {
        if ch == '\n' || i == input_text.len() - 1 {
            let visible_border = cmp::min((state.config.max_col() - 2) as usize, buffer.len());
            let visible_range = &buffer[0..visible_border];

            let line = String::from_iter((&visible_range).iter());
            write!(state.stdout, "{}", line).unwrap();

            state.add_row(Some(buffer));
            state.move_cursor(1, 0);

            buffer = Vec::new();
        } else {
            buffer.push(ch);
        }
    }
}

fn interpret_char(c: char, state: &mut State) {
    print!("{}", c);

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
        Key::PageUp => state.go_to(state.config.min_row(), state.col()),
        Key::PageDown => state.go_to(state.active_rows, state.col()),
        Key::Backspace => (),
        Key::Alt('q') => die(state),
        _ => (),
    }
}

pub fn run(stdin: Stdin, mut state: &mut State) {
    for key in stdin.keys() {
        interpret_key(key.unwrap(), &mut state);
    }
}

use std::cmp;
use std::io::Write;
use std::iter::FromIterator;
use std::process;

use termion::event::Key;

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

    write!(state.stdout, "{}", termion::cursor::Goto(1,2)).unwrap();
    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();

    process::exit(0);
}

/* Write the welcome message in the terminal, as well as all the '~'.
 * Also, it handles input files given from the command line. */
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

    state.add_row(None);

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

/* Given the text from a external file (input_text), write it's contexts on the screen
 * and add the necessary rows in state, with its contents. This function was only tested
 * when called by start_term */
fn draw_file(state: &mut State, input_text: String) {
    let mut buffer: Vec<char> = Vec::new();
    for ch in input_text.chars() {
        if ch == '\n' {
            let visible_border = cmp::min((state.config.max_col() - 2) as usize, buffer.len());
            let visible_range = &buffer[0..visible_border];

            let line = String::from_iter((&visible_range).iter());
            write!(state.stdout, "{}", line).unwrap();

            write!(state.config.log, "{}", buffer.len()).unwrap();

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
        Key::PageUp => state.move_cursor(2 - state.row() as i16, 0),
        Key::PageDown => state.move_cursor(state.config.max_row() as i16, 0),
        Key::Backspace => (),
        Key::Alt('q') => die(state), // die(state),
        _ => (),
    }
}

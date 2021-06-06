use std::io::Stdin;
use std::io::Write;
use std::process;

use termion::event::Key;
use termion::input::TermRead;

use super::state::State;

/* Turn the terminal back from Raw mode and ends the program */
pub fn die(state: &mut State) {
    let goodbye_message: &str = "Good Bye!";
    let first_line_col: usize = (state.config.width() as usize - goodbye_message.len()) / 2;

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

    for row in 2..=state.config.height() as u16 {
        write!(state.stdout, "{}~", termion::cursor::Goto(1, row)).unwrap();
    }
    state.add_row(None);
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
    for line in input_text.lines() {
        let right_border = std::cmp::min(
            (state.config.width() - state.config.min_col() + 1) as usize,
            line.len(),
        );
        let visible_line = &line[..right_border];
        write!(state.stdout, "{}", visible_line).unwrap();
        state.add_row(Some(line.chars().collect()));
        state.move_cursor(1, 0);
    }
}

fn interpret_char(c: char, state: &mut State) {
    let index_to_add = (state.col() - state.config.min_col()) as usize;
    state.current_row().chars.insert(index_to_add, c);

    let curr_col_num = state.col();
    state.go_to(state.row(), state.config.min_col());

    let curr_row = state.current_row().clone();
    write!(
            state.stdout,
            "{}{}",
            termion::clear::UntilNewline,
            curr_row
          ).unwrap();

    state.go_to(state.row(), curr_col_num + 1);
}

fn interpret_enter(state: &mut State) {
    let current_position = (state.col() - state.config.min_col()) as usize;
    let line_length = state.current_row().chars.len();
    let chars_to_move: Vec<char> =
        state.current_row().chars[current_position..line_length].to_vec();
    let curr_row_num = state.row();
    state.current_row().chars.drain(current_position .. line_length);
    state.insert_row((state.row() + 1) as usize, Some(chars_to_move));
    state.re_draw();
    state.go_to(curr_row_num + 1, 0);
}

fn interpret_backspace(state: &mut State) {
    if state.col() > state.config.min_col() {
        let index_to_remove = (state.col() - state.config.min_col() - 1) as usize;
        state.current_row().chars.remove(index_to_remove);
    }

    let curr_col_num = state.col();
    state.go_to(state.row(), state.config.min_col());

    let curr_row = state.current_row().clone();
    write!(
            state.stdout,
            "{}{}",
            termion::clear::UntilNewline,
            curr_row
          ).unwrap();

    state.go_to(state.row(), curr_col_num - 1);
}

pub fn interpret_key(key: Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => interpret_enter(state),
        Key::Char(c) => interpret_char(c, state),
        Key::Backspace => interpret_backspace(state),
        Key::Left => state.move_cursor(0, -1),
        Key::Right => state.move_cursor(0, 1),
        Key::Up => state.move_cursor(-1, 0),
        Key::Down => state.move_cursor(1, 0),
        Key::PageUp => state.go_to(state.config.min_row(), state.col()),
        Key::PageDown => state.go_to(state.active_rows, state.col()),
        Key::Alt('q') => die(state),
        _ => (),
    }
}

pub fn run(stdin: Stdin, mut state: &mut State) {
    for key in stdin.keys() {
        interpret_key(key.unwrap(), &mut state);
    }
}

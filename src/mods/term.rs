use std::fs;
use std::io;
use std::fs::File;
use std::io::Stdin;
use std::io::Write;
use std::path::Path;
use std::process;

use termion::event::Key;
use termion::input::TermRead;

use super::state::State;

/* Turn the terminal back from Raw mode and ends the program */
pub fn die(state: &mut State) {
    let goodbye_message: &str = "Good Bye!";
    let first_line_col: usize =
          (state.config.width() as usize - goodbye_message.len()) / 2;

    write!(
        state.stdout,
        "{}{}{}{}",
        termion::cursor::Show,
        termion::clear::All,
        termion::cursor::Goto(first_line_col as u16, 1),
        goodbye_message
    ).unwrap();

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
    ).unwrap();

    for row in 1..=state.config.height() as u16 {
        write!( // cant use go_to here because the lines weren't initialized yet
            state.stdout,
            "{}~",
            termion::cursor::Goto(1, row)
        ).unwrap();
    }
    if let Some(file_name) = &mut state.config.file_name {
        let file_text = fs::read_to_string(file_name).unwrap();
        handle_file(state, file_text);
    } else {
        state.add_row(Vec::new());
    }
    state.go_to(state.config.min_row(), state.config.min_col());
    state.stdout.flush().unwrap();
}

fn save_file(state: &mut State) {
    let file_name = match &state.config.file_name {
        Some(file_name) => file_name.clone(),
        None            => prompt("Enter the file name: ", state)
    };
    let editor_text = state.get_all_text();
    let mut file = File::create(Path::new(&file_name)).unwrap();

    file.write(editor_text.as_bytes()).unwrap();
    set_message(&format!("File {} written.", file_name)[..], state);
}

fn handle_file(state: &mut State, input_text: String) {
    if input_text.is_empty() {
        state.add_row(Vec::new());
    } else {
        for line in input_text.lines() {
            state.add_row(line.chars().collect());
        }
    }
    state.re_draw();
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
    state.insert_row((state.row() + 1) as usize, chars_to_move);
    state.re_draw();
    state.go_to(curr_row_num + 1, 0);
}

fn interpret_backspace(state: &mut State) {
    if state.col() > state.config.min_col() {
        let index_to_remove = (state.col() - state.config.min_col() - 1) as usize;
        state.current_row().chars.remove(index_to_remove);

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
    } else if state.row() > state.config.min_row() {
        let chars_to_move = state.current_row().chars.clone();
        let curr_row = state.row();
        state.remove_row(state.row() as usize);
        state.move_cursor(-1, 0);
        let prev_row_len = state.row_length(state.row());
        state.current_row().chars.extend(chars_to_move.iter());
        state.re_draw();
        state.go_to(curr_row - 1, prev_row_len as u16);
    }
}

pub fn interpret_key(key: Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => interpret_enter(state),
        Key::Char(c)      => interpret_char(c, state),
        Key::Backspace    => interpret_backspace(state),
        Key::Left         => state.move_cursor(0, -1),
        Key::Right        => state.move_cursor(0, 1),
        Key::Up           => state.move_cursor(-1, 0),
        Key::Down         => state.move_cursor(1, 0),
        Key::PageUp       => state.go_to(state.config.min_row(), state.col()),
        Key::PageDown     => state.go_to(state.rows.len() as u16, state.col()),
        Key::Alt('s')     => save_file(state),
        Key::Alt('q')     => die(state),
        _                 => (),
    }
}

pub fn run(stdin: Stdin, mut state: &mut State) {
    for key in stdin.keys() {
        interpret_key(key.unwrap(), &mut state);
    }
}

pub fn set_message(msg: &str, state: &mut State) {
    let (row, col) = (state.row(), state.col());
    state.go_to_bottom();
    write!(state.stdout,
           "{}{}",
           termion::clear::UntilNewline,
           msg,
           ).unwrap();
    state.stdout.flush().unwrap();
    state.go_to(row, col);
}

pub fn prompt(msg: &str, state: &mut State) -> String {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut pointer: usize = 0;
    set_message(msg, state);
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\x0A') => { set_message("", state); break; }
            Key::Char(c)      => { buffer.insert(pointer, c); pointer += 1; }
            Key::Left         => { if pointer > 0            { pointer -= 1; } },
            Key::Right        => { if pointer < buffer.len() { pointer += 1; } },
            Key::Backspace    => { buffer.remove(pointer); pointer -= 1; }
            _ => ()
        }
        set_message(&format!("{}{}", msg, buffer)[..], state);
    }
    buffer
}

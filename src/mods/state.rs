use std::path::Path;
use std::fs::File;
use std::io::{stdout};
use std::fs;
use std::io::Write;

use termion::raw::IntoRawMode;

use super::config::Config;
use super::data::Data;
use super::term::Term;
use super::interface::run_prompt;

pub struct State {
    pub term: Term,
    pub data: Data,
    pub config: Config,
}

impl State {
    fn new(row: u16, col: u16, config: Config) -> State {
        let stdout = stdout().into_raw_mode().unwrap();
        State {
            term: Term::new(row, col, 0, 0, stdout),
            data: Data::from_vec(vec![String::new(); 1]),
            config,
        }
    }

    pub fn create(row: u16, col: u16, config: Config) -> State {
        let mut state = State::new(row, col, config);
        state.term.start(&state.config);
        state.handle_file();
        state
    }

    fn handle_file(&mut self) {
        if let Some(file_name) = &mut self.config.file_name {

            let input_text = fs::read_to_string(file_name).unwrap();
            if input_text.is_empty() {
                self.add_row(String::new());
            } else {
                for line in input_text.lines() {
                    self.add_row(line.chars().collect());
                }
            }
            self.term.draw_screen(&self.data, &self.config);
        } else {
            self.add_row(String::new());
        }
        self.go_to(self.config.min_row(), self.config.min_col());
    }
    
    pub fn save_file(&mut self) {
        let file_name = match &self.config.file_name {
            Some(file_name) => file_name.clone(),
            None            => run_prompt("Enter the file name: ", self)
        };
        let editor_text = self.data.to_string();
        let mut file = File::create(Path::new(&file_name)).unwrap();

        file.write(editor_text.as_bytes()).unwrap();
        self.set_message(&format!("File {} written.", file_name)[..]);
    }

    fn current_row(&mut self) -> &mut String {
        self.data.get_row(self.term.row)
    }

    fn insert_row(&mut self, index: u16, row: String) {
        self.data.insert(index, row.clone());
        self.term.insert_row(&self.data, &self.config);
    }

    fn insert_char(&mut self, row: u16, col: u16, c: char) {
        self.data.insert_char(row, col, c);
        self.term.draw_row(row, &self.data, &self.config);
    }

    pub fn place_char(&mut self, c: char) {
        self.insert_char(self.term.row, self.term.col, c);
        self.move_cursor(0, 1);
    }

    pub fn break_line(&mut self) {
        let col = self.term.col as usize; // avoid two uses of self in the same instruction
        let chars: String =
            self.current_row()[col ..].to_string();
        self.data.truncate_row(self.term.row, self.term.col);
        self.insert_row(self.term.row + 1, chars);
        self.term.draw_screen(&self.data, &self.config);
        self.go_to(self.term.row + 1, 0);
    }

    pub fn run_backspace(&mut self) {
        if self.term.col > self.config.min_col() {
            let rem_index = self.term.col - self.config.min_col() - 1;
            self.data.remove_char(self.term.row, rem_index);
            self.term.draw_row(self.term.row, &self.data, &self.config);
            self.go_to(self.term.row, self.term.col - 1);
        } else if self.term.row > self.config.min_row() {
            let prev_len = self.data.get_row(self.term.row - 1).len();
            let curr_text = self.current_row().clone();
            self.data.extend_row(self.term.row - 1, curr_text);
            self.data.remove(self.term.row);
            self.term.draw_screen(&self.data, &self.config);
            self.go_to(self.term.row - 1, prev_len as u16);
        }
    }

    // Insert row after the last one
    fn add_row(&mut self, to_add: String) {
        self.insert_row(self.data.len() as u16, to_add);
    }

    pub fn move_cursor(&mut self, row_delta: i16, col_delta: i16) {
        self.term.move_cursor(row_delta, col_delta, &self.data, &self.config);
    }

    fn go_to(&mut self, row: u16, col: u16) {
        self.term.go_to(row, col, &self.data, &self.config);
    }
    
    pub fn set_message(&mut self, msg: &str) {
        self.term.set_message(msg, &self.data, &self.config);
    }

    pub fn die(&mut self) {
        self.term.die(&self.config);
    }
}

use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

pub struct Row {
    pub chars: Vec<char>,
}

impl Clone for Row {
    fn clone(&self) -> Row {
        Row {
            chars: self.chars.clone(),
        }
    }
}

impl Row {
    pub fn length(&self) -> usize {
        self.chars.len() + 3 as usize // because each line stars with a ~ and a blank space
    }
    pub fn pop(&mut self) {
        self.chars.pop();
    }
    pub fn push(&mut self, c: char) {
        self.chars.push(c);
    }
}

pub struct State {
    pub row: usize,
    pub col: usize,
    pub max_row: usize,
    pub max_col: usize,
    pub rows: Vec<Row>,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>
}

impl State {
    pub fn new(row: usize, col: usize, max_row: usize, max_col: usize) -> State {
        let base_row = Row { chars: Vec::new(), };
        State {
            row,
            col,
            max_row,
            max_col,
            stdout: stdout().into_raw_mode().unwrap(),
            rows: vec![base_row; max_row as usize],
        }
    }

    fn fix_cursor_bounds(&mut self) {
        if self.row < 2 {
            self.row = 2;
        }
        if self.row >= self.max_row {
            self.row = self.max_row - 1;
        }
        if self.col < 3 {
            self.col = 3;
        }
        if self.col > self.current_row().length() { // make sure that row_length <= max_col
            self.col = self.current_row().length();
        }

    }

    pub fn move_cursor(&mut self, row_delta: i8, col_delta: i8) {
        self.row = ((self.row as i8) + row_delta) as usize;
        self.col = ((self.col as i8) + col_delta) as usize;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}", termion::cursor::Goto(self.col as u16, self.row as u16)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn current_row(&mut self) -> &mut Row {
        &mut self.rows[self.row]
    }
}


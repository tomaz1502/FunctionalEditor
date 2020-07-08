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
    pub fn new() -> Row {
        Row { chars: Vec::new(), }
    }
    pub fn from_source(source: Vec<char>) -> Row {
        Row { chars: source, }
    }
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
    row: usize,
    col: usize,
    pub max_row: usize,
    pub max_col: usize,
    pub active_rows: usize,
    pub rows: Vec<Row>,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>
}

impl State {
    pub fn new(row: usize, col: usize, max_row: usize, max_col: usize) -> State {
        State {
            row,
            col,
            max_row,
            max_col,
            active_rows: 2,
            stdout: stdout().into_raw_mode().unwrap(),
            rows: vec![Row::new(); 3],
        }
    }

    fn fix_cursor_bounds(&mut self) {
        if self.row < 2 {
            self.row = 2;
        }
        if self.row > self.active_rows {
            self.row = self.active_rows;
        }
        if self.col < 3 {
            self.col = 3;
        }
        if self.col > self.current_row().length() { // make sure that row_length <= max_col
            self.col = self.current_row().length();
        }

    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn current_row(&mut self) -> &mut Row {
        &mut self.rows[self.row]
    }

    pub fn move_cursor(&mut self, row_delta: i8, col_delta: i8) {
        self.row = ((self.row as i8) + row_delta) as usize;
        self.col = ((self.col as i8) + col_delta) as usize;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}", termion::cursor::Goto(self.col as u16, self.row as u16)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn go_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}", termion::cursor::Goto(col as u16, row as u16)).unwrap();
        self.stdout.flush().unwrap();
    }
}


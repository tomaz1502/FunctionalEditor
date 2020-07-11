use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use termion::color;

use super::config;
use super::row::Row;

pub struct State {
    row: usize,
    col: usize,
    pub vert_offset: usize,
    pub active_rows: usize,
    pub rows: Vec<Row>,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
    pub config: config::Config,
}

impl State {
    pub fn new(row: usize, col: usize, config: config::Config) -> State {
        State {
            row,
            col,
            config,
            vert_offset: 0,
            active_rows: 2,
            stdout: stdout().into_raw_mode().unwrap(),
            rows: vec![Row::new(); 3],
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row_length(&self, row: usize) -> usize {
        self.rows[row].chars.len() + self.config.left_most()
    }

    pub fn current_row(&mut self) -> &mut Row {
        &mut self.rows[self.row]
    }

    pub fn add_row(&mut self, to_add: Option<Vec<char>>) {
        match to_add {
            Some(row) => self.rows.push(Row::from_source(row)),
            None      => self.rows.push(Row::new()),
        }
        self.active_rows += 1;
        if self.active_rows <=  self.config.max_row() + self.vert_offset {
            write!(self.stdout, "{}{}{}{}",
                                 color::Fg(color::Yellow),
                                 termion::cursor::Goto(1, self.active_rows as u16),
                                 self.active_rows - 1,
                                 color::Fg(color::Reset)).
                                 unwrap();
        }
    }

    fn fix_cursor_bounds(&mut self) {
        if self.row < 2 {
            self.row = 2;
        }

        if self.row > self.active_rows {
            self.row = self.active_rows;
        }

        if self.row > self.config.max_row() + self.vert_offset {
            self.vert_offset = self.row - self.config.max_row();
            self.re_draw();
        }

        else if self.row - 2 < self.vert_offset {
            self.vert_offset = self.row - 2;
            self.re_draw();
        }

        if self.col < self.config.left_most() {
            self.col = self.config.left_most();
        }

        if self.col > self.row_length(self.row) { // make sure that row_length <= max_col
            self.col = self.row_length(self.row);
        }

    }

    pub fn move_cursor(&mut self, row_delta: i8, col_delta: i8) {
        let new_row = ((self.row as i8) + row_delta) as usize;
        let new_col = ((self.col as i8) + col_delta) as usize;
        self.go_to(new_row, new_col);
    }

    pub fn go_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}",
               termion::cursor::Goto(self.col as u16, (self.row - self.vert_offset) as u16)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn re_draw(&mut self) {

        for row in 2 .. (self.config.max_row() + 1) as usize  {
            write!(self.stdout, "{}{}",
                   termion::cursor::Goto(1, row as u16),
                   termion::clear::UntilNewline).
                   unwrap();
            write!(self.stdout, "{}{}{}",
                   termion::color::Fg(termion::color::Yellow),
                   row + self.vert_offset - 1,
                   termion::color::Fg(termion::color::Reset)).
                   unwrap();
            write!(self.stdout, "{}{}",
                   termion::cursor::Goto(self.config.left_most() as u16, row as u16),
                   self.rows[row + self.vert_offset]).
                   unwrap();
        }
        self.stdout.flush().unwrap();
    }
}


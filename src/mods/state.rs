use std::io::{stdout, Write};
use termion::color;
use termion::raw::IntoRawMode;

use super::config;
use super::row::Row;

pub struct State {
    row: u16,
    col: u16,
    pub vert_offset: u16,
    pub hor_offset: u16,
    pub active_rows: u16,
    pub rows: Vec<Row>,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
    pub config: config::Config,
}

impl State {
    pub fn new(row: u16, col: u16, config: config::Config) -> State {
        State {
            row,
            col,
            config,
            vert_offset: 0,
            hor_offset: 0,
            active_rows: 0,
            stdout: stdout().into_raw_mode().unwrap(),
            rows: vec![Row::new(); 3],
        }
    }

    pub fn row(&self) -> u16 {
        self.row
    }

    pub fn col(&self) -> u16 {
        self.col
    }

    pub fn row_length(&self, row: u16) -> u16 {
        self.rows[row as usize].chars.len() as u16 + self.config.min_col()
    }

    pub fn current_row(&mut self) -> &mut Row {
        &mut self.rows[self.row as usize]
    }

    pub fn add_row(&mut self, to_add: Option<Vec<char>>) {
        match to_add {
            Some(row) => self.rows.push(Row::from_vec(row)),
            None => self.rows.push(Row::new()),
        }

        self.active_rows += 1;
        if self.active_rows <= self.config.max_row() + self.vert_offset {
            write!(
                self.stdout,
                "{}{}{}{}",
                color::Fg(color::Yellow),
                termion::cursor::Goto(1, self.active_rows as u16),
                self.active_rows,
                color::Fg(color::Reset)
            )
            .unwrap();
        }
    }

    /* Make sure that self.row and self.col is on a valid position of the file.
     * In case it get off the screen we increase the offset and re_draw (scroll). */
    fn fix_cursor_bounds(&mut self) {
        if self.row < self.config.min_row() {
            self.row = self.config.min_row();
        }

        if self.row > self.active_rows {
            self.row = self.active_rows;
        }

        if self.row > self.config.max_row() + self.vert_offset {
            self.vert_offset = self.row - self.config.max_row();
            self.re_draw();
        } else if self.row - self.config.min_row() < self.vert_offset {
            self.vert_offset = self.row - self.config.min_row();
            self.re_draw();
        }

        if self.col < self.config.min_col() {
            self.col = self.config.min_col();
        }

        if self.col > self.row_length(self.row) {
            self.col = self.row_length(self.row);
        }

        if self.col > self.config.max_col() + self.hor_offset {
            self.hor_offset = self.col - self.config.max_col();
            self.re_draw();
        } else if self.col < self.hor_offset + self.config.min_col() {
            self.hor_offset = self.col - self.config.min_col(); // ?
            self.re_draw();
        }
    }

    pub fn move_cursor(&mut self, row_delta: i16, col_delta: i16) {
        let new_row = ((self.row as i16) + row_delta) as u16;
        let new_col = ((self.col as i16) + col_delta) as u16;
        self.go_to(new_row, new_col);
    }

    pub fn go_to(&mut self, row: u16, col: u16) {
        self.row = row;
        self.col = col;
        self.fix_cursor_bounds();
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(self.col - self.hor_offset,
                                  self.row - self.vert_offset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn re_draw(&mut self) {
        for row in self.config.min_row()..=self.config.max_row() {
            if row > self.active_rows {
                break;
            }

            let mut line_print = String::new();
            let active_row = &self.rows[(row + self.vert_offset) as usize];

            if active_row.chars.len() > self.hor_offset as usize {
                let left_border = self.hor_offset as usize;
                let right_border =
                    std::cmp::min(
                      left_border + (self.config.max_col() - self.config.min_col()) as usize,
                      active_row.chars.len()
                    );

                line_print = active_row.chars[left_border..right_border].iter()
                                                                        .collect::<String>();
            }

            write!(
                self.stdout,
                "{}{}{}{}{}",
                termion::cursor::Goto(1, row as u16),
                termion::clear::UntilNewline,
                termion::color::Fg(termion::color::Yellow),
                row + self.vert_offset,
                termion::color::Fg(termion::color::Reset)
            )
            .unwrap();
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(self.config.min_col() as u16, row as u16),
                line_print
            )
            .unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

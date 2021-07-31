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
            stdout: stdout().into_raw_mode().unwrap(),
            rows: vec![Row::new(); 1], // rows are 1-based
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

    pub fn remove_row(&mut self, index: usize) {
        if self.rows.len() as u16 <= self.config.height() + self.vert_offset {
            write!(
                self.stdout,
                "{}~",
                termion::cursor::Goto(1, self.rows.len() as u16 - 1),
            ).unwrap();
        }
        self.rows.remove(index);
    }

    pub fn insert_row(&mut self, index: usize, row: Vec<char>) {
        self.rows.insert(index, Row::from_vec(row));

        if self.rows.len() as u16 <= self.config.height() + self.vert_offset {
            write!(
                self.stdout,
                "{}{}{}{}",
                color::Fg(color::Yellow),
                termion::cursor::Goto(1, self.rows.len() as u16 - 1),
                self.rows.len() - 1,
                color::Fg(color::Reset)
            ).unwrap();
        }
    }

    // Insert row after the last one
    pub fn add_row(&mut self, to_add: Vec<char>) {
        self.insert_row(self.rows.len(), to_add);
    }

    /* Make sure that self.row and self.col is on a valid position of the file.
     * In case it get off the screen we increase the offset and re_draw (scroll). */
    fn fix_cursor_bounds(&mut self) {
        if self.row < self.config.min_row() {
            self.row = self.config.min_row();
        }

        if self.row >= self.rows.len() as u16 {
            self.row = self.rows.len() as u16 - 1;
        }

        if self.row > self.config.height() + self.vert_offset {
            self.vert_offset = self.row - self.config.height();
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

        if self.col > self.config.width() + self.hor_offset {
            self.hor_offset = self.col - self.config.width();
            self.re_draw();
        } else if self.col < self.hor_offset + self.config.min_col() {
            self.hor_offset = self.col - self.config.min_col(); // ?
            self.re_draw();
        }
    }

    // here we dont assign to self.row/col, this way we can continue writing
    // from the same place as before
    pub fn go_to_bottom(&mut self) {
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(1, self.config.height() + 2)
        ).unwrap();
        self.stdout.flush().unwrap(); 
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
            termion::cursor::Goto(self.col - self.hor_offset, self.row - self.vert_offset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn re_draw(&mut self) {
        let curr_row = self.row();
        let curr_col = self.col();
        let num_rows = std::cmp::min(
                         self.config.height(),
                         self.rows.len() as u16 - 1
                       );
        for row in 1..=num_rows {
            // iterating through visible rows
            write!(
                self.stdout,
                "{}{}{}{}{}",
                termion::cursor::Goto(1, row as u16),
                termion::clear::UntilNewline,
                termion::color::Fg(termion::color::Yellow),
                row + self.vert_offset,
                termion::color::Fg(termion::color::Reset)
            ).unwrap();

            let active_row = &self.rows[(row + self.vert_offset) as usize];
            if active_row.chars.len() > self.hor_offset as usize {
                let left_border = self.hor_offset as usize;
                let right_border = std::cmp::min(
                    left_border + (self.config.width() - self.config.min_col()) as usize,
                    active_row.chars.len(),
                );

                let line_print: String =
                    active_row.chars[left_border..right_border].iter().collect();
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(self.config.min_col() as u16, row as u16),
                    line_print
                )
                .unwrap();
            }
        }
        for row in self.rows.len() as u16..=self.config.height() {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(2, row),
                termion::clear::UntilNewline
            ).unwrap();
        }
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(curr_col, curr_row)
        ).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn get_all_text(&self) -> String {
        self.rows.clone()[1..].to_vec()
        .into_iter()
        .map(|row| row.chars.into_iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
    }
}


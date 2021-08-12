use std::io::{Write};
use std::process;

use termion::color;

use super::config::Config;
use super::data::Data;

pub struct Term {
    pub row: u16,
    pub col: u16,
    pub vert_offset: u16,
    pub hor_offset: u16,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Term {
    pub fn new(row: u16,
               col: u16,
               vert_offset: u16,
               hor_offset: u16,
               stdout: termion::raw::RawTerminal<std::io::Stdout>) -> Term {
        Term {
            row,
            col,
            vert_offset,
            hor_offset,
            stdout,
        }
    }

    pub fn start(&mut self, config: &Config) {
        write!(self.stdout,
               "{}{}",
               termion::clear::All,
               termion::cursor::Show,
              ).unwrap();

        for row in 1..=config.height() as u16 {
            write!(self.stdout,
                   "{}~",
                   termion::cursor::Goto(1, row)
                  ).unwrap();
        }
    }

    fn adjust_col(&self, col: u16, config: &Config) -> u16 {
        col + config.min_col() - self.hor_offset
    }

    fn adjust_row(&self, row: u16, config: &Config) -> u16 {
        row + config.min_row() - self.vert_offset
    }

    /* Make sure that self.row and self.col is on a valid position of the file.
     * In case it get off the screen we increase the offset and re_draw (scroll). */
    fn fix_cursor_bounds(&mut self, data: &Data, config: &Config) {
        let mut changed_offset = false;

        if self.row >= data.len() as u16 {
            self.row = data.len() as u16 - 1;
        }

        if self.row > config.height() + self.vert_offset {
            self.vert_offset = self.row - config.height();
            changed_offset = true;
        } else if self.row < self.vert_offset {
            self.vert_offset = self.row;
            changed_offset = true;
        }

        if self.col > data.row_length(self.row) as u16 {
            self.col = data.row_length(self.row) as u16;
        }

        if self.col > config.width() + self.hor_offset {
            self.hor_offset = self.col - config.width();
            changed_offset = true;
        } else if self.col < self.hor_offset {
            self.hor_offset = self.col;
            changed_offset = true;
        }

        if changed_offset {
            self.draw_screen(data, config);
        }
    }

    // pub fn pop_row(&mut self, index: u16, data: &Data, config: &Config) {
    //     if data.len() as u16 <= config.height() + self.vert_offset {
    //         write!(
    //             self.stdout,
    //             "{}~",
    //             termion::cursor::Goto(1, data.len() as u16 - 1),
    //         ).unwrap();
    //     }
    // }

    pub fn add_row(&mut self, data: &Data, config: &Config) {
        if data.len() as u16 <= config.height() + self.vert_offset {
            write!(self.stdout,
                   "{}{}{}{}",
                   color::Fg(color::Yellow),
                   termion::cursor::Goto(1, data.len() as u16),
                   data.len(),
                   color::Fg(color::Reset)
                  ).unwrap()
        }
    }

    pub fn draw_row(&mut self, row: u16, data: &Data, config: &Config) {
        let (curr_row, curr_col) = (self.row, self.col);
        let curr_text = data.get_row_const(row);
        self.go_to(row, 0, data, config);
        write!(self.stdout,
               "{}{}",
               termion::clear::UntilNewline,
               curr_text
              ).unwrap();
        self.go_to(curr_row, curr_col, data, config);
    }
    
    // here we dont assign to self.row/col, this way we can continue writing
    // from the same place as before
    pub fn go_to_bottom(&mut self, config: &Config) {
        write!(self.stdout,
               "{}",
               termion::cursor::Goto(1, config.height() + 2)
              ).unwrap();
        self.stdout.flush().unwrap(); 
    }

    pub fn move_cursor(&mut self, row_delta: i16, col_delta: i16, data: &Data, config: &Config) {
        let real_col_delta = std::cmp::max(col_delta, -(self.col as i16));
        let real_row_delta = std::cmp::max(row_delta, -(self.row as i16));
        let new_row = ((self.row as i16) + real_row_delta) as u16;
        let new_col = ((self.col as i16) + real_col_delta) as u16;
        self.go_to(new_row, new_col, data, config);
    }

    pub fn go_to(&mut self, row: u16, col: u16, data: &Data, config: &Config) {
        self.row = row;
        self.col = col;
        self.fix_cursor_bounds(data, config);
        let term_col = self.adjust_col(self.col, config);
        let term_row = self.adjust_row(self.row, config);
        write!(self.stdout,
               "{}",
               termion::cursor::Goto(term_col, term_row)
              ).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn draw_screen(&mut self, data: &Data, config: &Config) {
        let (curr_row, curr_col) = (self.row, self.col);
        for row in 0 .. data.len() {
            self.draw_row(row as u16, data, config);
        }
        for row in data.len() ..= config.height() as usize {
            write!(self.stdout,
                   "{}{}~",
                   termion::cursor::Goto(1, row as u16 + 1),
                   termion::clear::UntilNewline
                  ).unwrap();
        }
        self.go_to(curr_row, curr_col, data, config);
    }

    pub fn set_message(&mut self, msg: &str, data: &Data, config: &Config) {
        self.go_to_bottom(config);
        write!(self.stdout,
               "{}{}",
               termion::clear::UntilNewline,
               msg,
              ).unwrap();
        self.stdout.flush().unwrap();
        self.go_to(self.row, self.col, data, config);
    }

    /* Turn the terminal back from Raw mode and ends the program */
    pub fn die(&mut self, config: &Config) {
        let goodbye_message: &str = "Good Bye!";
        let first_line_col: usize =
              (config.width() as usize - goodbye_message.len()) / 2;

        write!(self.stdout,
               "{}{}{}{}",
               termion::cursor::Show,
               termion::clear::All,
               termion::cursor::Goto(first_line_col as u16, 1),
               goodbye_message
              ).unwrap();

        write!(self.stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();
        self.stdout.flush().unwrap();
        self.stdout.suspend_raw_mode().unwrap();

        process::exit(0);
    }
}

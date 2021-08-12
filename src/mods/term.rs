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

    /* Make sure that self.row and self.col is on a valid position of the file.
     * In case it get off the screen we increase the offset and re_draw (scroll). */
    fn fix_cursor_bounds(&mut self, data: &Data, config: &Config) {
        let mut must_redraw = false;

        if self.row < config.min_row() {
            self.row = config.min_row();
        }

        if self.row >= data.len() as u16 {
            self.row = data.len() as u16 - 1;
        }

        if self.row > config.height() + self.vert_offset {
            self.vert_offset = self.row - config.height();
            must_redraw = true;
        } else if self.row - config.min_row() < self.vert_offset {
            self.vert_offset = self.row - config.min_row();
            must_redraw = true;
        }

        if self.col < config.min_col() {
            self.col = config.min_col();
        }
 
        // println!("{}", data.row_length(self.row));
        if self.col > data.row_length(self.row) as u16 {
            self.col = data.row_length(self.row) as u16;
        }

        if self.col > config.width() + self.hor_offset {
            self.hor_offset = self.col - config.width();
            must_redraw = true;
        } else if self.col < self.hor_offset + config.min_col() {
            self.hor_offset = self.col - config.min_col(); // ?
            must_redraw = true;
        }

        // if must_redraw {
        //     self.draw_screen(data, config);
        // }
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

    pub fn insert_row(&mut self, data: &Data, config: &Config) {
        if data.len() as u16 <= config.height() + self.vert_offset {
            write!(self.stdout,
                   "{}{}{}{}",
                   color::Fg(color::Yellow),
                   termion::cursor::Goto(1, data.len() as u16 - 1),
                   data.len() - 1,
                   color::Fg(color::Reset)
                  ).unwrap()
        }
    }

    pub fn draw_row(&mut self, row: u16, data: &Data, config: &Config) {
        self.go_to(row, config.min_col(), data, config);
        let curr_row = data.get_row_const(row);
        write!(self.stdout,
               "{}{}",
               termion::clear::UntilNewline,
               curr_row
              ).unwrap();
        self.go_to(self.row, self.col, data, config);
    }
    
    // here we dont assign to self.row/col, this way we can continue writing
    // from the same place as before
    pub fn go_to_bottom(&mut self, config: &Config) {
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(1, config.height() + 2)
        ).unwrap();
        self.stdout.flush().unwrap(); 
    }

    pub fn move_cursor(&mut self, row_delta: i16, col_delta: i16, data: &Data, config: &Config) {
        let new_row = ((self.row as i16) + row_delta) as u16;
        let new_col = ((self.col as i16) + col_delta) as u16;
        self.go_to(new_row, new_col, data, config);
    }

    pub fn go_to(&mut self, row: u16, col: u16, data: &Data, config: &Config) {
        self.row = row;
        self.col = col;
        self.fix_cursor_bounds(data, config);
        write!(self.stdout,
               "{}",
               termion::cursor::Goto(self.col - self.hor_offset, self.row - self.vert_offset)
              ).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn draw_screen(&mut self, data: &Data, config: &Config) {
        let curr_row = self.row;
        let curr_col = self.col;
        let num_rows = std::cmp::min(config.height(), data.len() as u16 - 1);
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

            let active_row = data.get_row_const(row + self.vert_offset);
            if active_row.len() > self.hor_offset as usize {
                let left_border = self.hor_offset as usize;
                let right_border = std::cmp::min(
                    left_border + (config.width() - config.min_col()) as usize,
                    active_row.len(),
                );

                let line_print: String =
                    active_row[left_border..right_border].to_string();
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(config.min_col() as u16, row as u16),
                    line_print
                )
                .unwrap();
            }
        }
        for row in data.len() as u16..=config.height() {
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

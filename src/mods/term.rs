use std::io::{Write};
use std::process;

use termion::*;

use super::config::Config;
use super::data::Data;
use super::lib;

pub struct Term {
    pub row     : u16,
    pub col     : u16,
    vert_offset : u16,
    hor_offset  : u16,
    stdout      : raw::RawTerminal<std::io::Stdout>,
}

impl Term {
    pub fn new(row: u16,
               col: u16,
               vert_offset: u16,
               hor_offset: u16,
               stdout: raw::RawTerminal<std::io::Stdout>) -> Term {
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
               clear::All,
               cursor::Show,
              ).unwrap();

        for row in 1..=config.height() as u16 {
            write!(self.stdout,
                   "{}~",
                   cursor::Goto(1, row)
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
    
        if self.row >= config.height() + self.vert_offset {
            self.vert_offset = self.row - config.height() + 1;
            changed_offset = true;
        } else if self.row < self.vert_offset {
            self.vert_offset = self.row;
            changed_offset = true;
        }

        if self.col > data.row_length(self.row) as u16 {
            self.col = data.row_length(self.row) as u16;
        }

        if self.col + config.min_col() > config.width() + self.hor_offset {
            self.hor_offset = self.col + config.min_col() - config.width();
            changed_offset = true;
        } else if self.col < self.hor_offset {
            self.hor_offset = self.col;
            changed_offset = true;
        }

        if changed_offset {
            self.draw_text(data, config);
        }
    }


    pub fn add_row(&mut self, data: &Data, config: &Config) {
        if data.len() as u16 <= config.height() + self.vert_offset {
            write!(self.stdout,
                   "{}{}{}{}",
                   color::Fg(color::Yellow),
                   cursor::Goto(1, data.len() as u16),
                   data.len(),
                   color::Fg(color::Reset)
                  ).unwrap()
        }
    }

    fn set_color(&mut self, from: &str) {
        match from {
            "yellow" => write!(self.stdout, "{}", color::Fg(color::Yellow)).unwrap(),
            "green"  => write!(self.stdout, "{}", color::Fg(color::Green)).unwrap(),
            "red"    => write!(self.stdout, "{}", color::Fg(color::Red)).unwrap(),
            "blue"   => write!(self.stdout, "{}", color::Fg(color::Blue)).unwrap(),
            "white"  => write!(self.stdout, "{}", color::Fg(color::White)).unwrap(),
            "normal" => write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap(),
            _        => panic!("unknown color"),
        }
    }

    // Assumes row < data.len()
    pub fn draw_row(&mut self, row: u16, data: &Data, config: &Config) {
        let row_len = data.row_length(row);
        let curr_text: &str =
            if row_len >= self.hor_offset as usize {
                let right_border =
                    std::cmp::min((self.hor_offset + config.width()) as usize,
                                  row_len);
                &data.get_row(row)[self.hor_offset as usize .. right_border]
            } else {
                ""
            };
        write!(self.stdout,
               "{}{}{}{}{}{}",
               cursor::Goto(1, self.adjust_row(row, config)),
               clear::UntilNewline,
               color::Fg(color::Yellow),
               row + 1,
               color::Fg(color::Reset),
               cursor::Goto(config.min_col(), self.adjust_row(row, config))
              ).unwrap();
        for (word, whites) in lib::words_and_spaces(curr_text) {
            self.set_color(&config.color_from_word(&word));
            write!(self.stdout, "{}{}", word, whites).unwrap();
        }
        self.rewind(data, config);
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
               cursor::Goto(term_col, term_row)
              ).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn rewind(&mut self, data: &Data, config: &Config) {
        self.go_to(self.row, self.col, data, config);
    }

    pub fn draw_text(&mut self, data: &Data, config: &Config) {
        let non_empty_rows = std::cmp::min(self.vert_offset + config.height()
                                          ,data.len() as u16);
        for row in self.vert_offset .. non_empty_rows {
                self.draw_row(row, data, config);
        }
        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
        for row in data.len() as u16 .. self.vert_offset + config.height() {
            write!(self.stdout,
                   "{}{}~",
                   cursor::Goto(1, self.adjust_row(row, config)),
                   clear::UntilNewline
                  ).unwrap();
        }
        self.rewind(data, config);
    }

    pub fn set_message(&mut self, msg: &str, data: &Data, config: &Config) {
        write!(self.stdout,
               "{}{}{}{}",
               cursor::Goto(1, config.height() + 2),
               clear::UntilNewline,
               color::Fg(color::Reset),
               msg,
              ).unwrap();
        self.rewind(data, config);
    }

    pub fn draw_status_line(&mut self, data: &Data, config: &Config) {
        let displayed_name = if config.file_name().is_empty() {
            "[No Name]".to_string()
        } else {
            config.file_name().clone()
        };

        let pos_info = "L: ".to_string()       +
                       &self.row.to_string()   +
                       "/"                     +
                       &data.len().to_string() +
                       "  |  C: "              +
                       &self.col.to_string()   +
                       "/"                     +
                       &data.row_length(self.row).to_string();

        if config.width() <= (displayed_name.len() + pos_info.len()) as u16 {
            panic!("terminal too thin!");
        }

        let rem_space =
          config.width() as usize - displayed_name.len() - pos_info.len();
        let middle: String =
            std::iter::repeat(" ").take(rem_space as usize).collect();

        let text = displayed_name + &middle + &pos_info;
        write!(self.stdout,
               "{}{}{}{}{}{}{}",
               cursor::Goto(1, config.height() + 1),
               clear::UntilNewline,
               color::Bg(color::White),
               color::Fg(color::Black),
               text,
               color::Bg(color::Reset),
               color::Fg(color::Reset),
              ).unwrap();
        self.rewind(data, config);
    }

    /* Turn the terminal back from Raw mode and ends the program */
    pub fn die(&mut self, config: &Config) {
        let goodbye_message: &str = "Good Bye!";
        let first_line_col: usize =
              (config.width() as usize - goodbye_message.len()) / 2;
        write!(self.stdout,
               "{}{}{}{}{}",
               cursor::Show,
               clear::All,
               cursor::Goto(first_line_col as u16, 1),
               color::Fg(color::Reset),
               goodbye_message
              ).unwrap();

        write!(self.stdout, "{}", cursor::Goto(1, 2)).unwrap();
        self.stdout.flush().unwrap();
        self.stdout.suspend_raw_mode().unwrap();

        process::exit(0);
    }
}

use std::io::Write;

pub struct State {
    pub row: u16,
    pub col: u16,
    pub max_row: u16,
    pub max_col: u16,
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>
}

impl State {
    pub fn fix_cursor_bounds(&mut self) {
        if self.row < 2 {
            self.row = 2;
        }
        if self.row >= self.max_row {
            self.row = self.max_row - 1;
        }
        if self.col < 2 {
            self.col = 2;
        }
        if self.col >= self.max_col {
            self.col = self.max_col - 1;
        }

    }
    pub fn move_cursor(&mut self, row_delta: i8, col_delta: i8) {
        self.row = ((self.row as i8) + row_delta) as u16;
        self.col = ((self.col as i8) + col_delta) as u16;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}", termion::cursor::Goto(self.col, self.row)).unwrap();
        self.stdout.flush().unwrap();
    }
}


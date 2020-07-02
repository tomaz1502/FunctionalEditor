#![allow(unused_imports)]
#![allow(dead_code)]

use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

struct State {
    row: u16,
    col: u16,
    max_row: u16,
    max_col: u16,
    stdout: termion::raw::RawTerminal<std::io::Stdout>
}

impl State {
    fn fix_cursor_bounds(&mut self) {
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
    fn move_cursor(&mut self, row_delta: i8, col_delta: i8) {
        self.row = ((self.row as i8) + row_delta) as u16;
        self.col = ((self.col as i8) + col_delta) as u16;
        self.fix_cursor_bounds();
        write!(self.stdout, "{}", termion::cursor::Goto(self.col, self.row)).unwrap();
        self.stdout.flush().unwrap();
    }
}

fn die(state: &mut State) {
    write!(state.stdout, "{}{}{}Goodbye!!!!!!!!!!!!!!",
                           termion::cursor::Show,
                           termion::clear::All,
                           termion::cursor::Goto(1,1))
                           .unwrap();
    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();
    process::exit(0);
}

fn start_term(state: &mut State) {
    write!(state.stdout, "{}{}{}Welcome!!!!",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           termion::cursor::Hide)
           .unwrap();

    for j in 2 .. state.max_row {
        write!(state.stdout, "{}", termion::cursor::Goto(1, j)).unwrap();
        println!("~");
    }

    write!(state.stdout, "{}", termion::cursor::Goto(2, 2)).unwrap();
    state.stdout.flush().unwrap();
}

fn interpret_key(key : Key, state: &mut State) {
    match key {
        Key::Char(c)   => { println!("{}", c); state.move_cursor(0, 1); },
        Key::Alt('q')  => die(state),
        Key::Alt(c)    => println!("M-{}", c),
        Key::Ctrl(c)   => println!("C-{}", c),
        Key::Left      => println!("<left>"),
        Key::Down      => println!("<down>"),
        Key::Up        => println!("<up>"),
        Key::Right     => println!("<right>"),
        _              => println!("wat"),
    }
}


fn main() {
    let stdin = stdin();

    let (width, height) = termion::terminal_size().unwrap();

    let mut state = State { row: 2,
                            col: 2,
                            max_row: height,
                            max_col: width,
                            stdout: stdout().into_raw_mode().unwrap()};

    start_term(&mut state);
    // termion::async_stdin();

    for key in stdin.keys() {
        // write!(state.stdout, "{}{}", termion::cursor::Goto(2,2), termion::clear::UntilNewline).unwrap();
        interpret_key(key.unwrap(), &mut state);
    }

    die(&mut state);
}

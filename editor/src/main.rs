#![allow(unused_imports)]

use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // termion::async_stdin();
    let (width, height) = termion::terminal_size().unwrap();

    write!(stdout, "{}{}{}Welcome!!!!",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           termion::cursor::Hide)
           .unwrap();

    stdout.flush().unwrap();

    for key in stdin.keys() {
        write!(stdout, "{}{}", termion::cursor::Goto(1,2), termion::clear::CurrentLine).unwrap();
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Char(c)   => println!("{}", c),
            Key::Alt(c)    => println!("M-{}", c),
            Key::Ctrl(c)   => println!("C-{}", c),
            Key::Left      => println!("<left>"),
            Key::Down      => println!("<down>"),
            Key::Up        => println!("<up>"),
            Key::Right     => println!("<right>"),
            _              => println!("wat"),
        };
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    // println!("Terminal Height: {}\nTerminal Width: {}", height, width);
}

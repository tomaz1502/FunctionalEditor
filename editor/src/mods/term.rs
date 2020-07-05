use std::process;
use std::io::Write;
use termion::event::Key;
use super::state::State;

pub fn die(state: &mut State) {
    write!(state.stdout, "{}{}{}Goodbye!!!!!!!!!!!!!!",
                           termion::cursor::Show,
                           termion::clear::All,
                           termion::cursor::Goto(1,1))
                           .unwrap();
    state.stdout.flush().unwrap();
    state.stdout.suspend_raw_mode().unwrap();
    process::exit(0);
}

pub fn start_term(state: &mut State) {
    write!(state.stdout, "{}{}{}Welcome!!!!",
           termion::clear::All,
           termion::cursor::Goto(1,1),
           termion::cursor::Hide)
           .unwrap();

    for j in 2 .. state.max_row {
        write!(state.stdout, "{}", termion::cursor::Goto(1, j)).unwrap();
        println!("~");
    }

    write!(state.stdout, "{}", termion::cursor::Goto(state.col, state.row)).unwrap();
    state.stdout.flush().unwrap();
}

pub fn interpret_key(key : Key, state: &mut State) {
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

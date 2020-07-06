use std::process;
use std::io::Write;
use termion::event::Key;
use super::state::State;
use super::state::Row;

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
           termion::cursor::Show)
           .unwrap();

    for j in 2 .. state.max_row as u16 {
        write!(state.stdout, "{}", termion::cursor::Goto(1, j)).unwrap();
        println!("~");
    }

    write!(state.stdout, "{}", termion::cursor::Goto(state.col as u16, state.row as u16)).unwrap();
    state.stdout.flush().unwrap();
}

fn interpret_char(c: char, state: &mut State) {
    print!("{}", c);
    if state.current_row().length() > state.max_col - 2 {
        state.current_row().pop();
    }
    state.current_row().push(c);
    state.move_cursor(0, 1);
}

fn interpret_enter(state: &mut State) {
    state.rows.push(Row::new());
    state.active_rows += 1;
    state.move_cursor(1, 0);
}

pub fn interpret_key(key : Key, state: &mut State) {
    match key {
        Key::Char('\x0A') => interpret_enter(state),
        Key::Char(c)      => interpret_char(c, state),
        Key::Left         => state.move_cursor(0, -1),
        Key::Right        => state.move_cursor(0,  1),
        Key::Up           => state.move_cursor(-1, 0),
        Key::Down         => state.move_cursor(1,  0),
        Key::Alt('q')     => die(state),
        _                 => (),
    }
}

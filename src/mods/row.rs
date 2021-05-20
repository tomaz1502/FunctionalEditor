use std::fmt;

pub struct Row {
    pub chars: Vec<char>,
}

impl Clone for Row {
    fn clone(&self) -> Row {
        Row {
            chars: self.chars.clone(),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_print = String::new();
        for &c in &self.chars {
            to_print.push(c);
        }
        write!(f, "{}", to_print)
    }
}

impl Row {
    pub fn new() -> Row {
        Row { chars: Vec::new(), }
    }
    pub fn from_vec(source: Vec<char>) -> Row {
        Row { chars: source, }
    }
    pub fn push(&mut self, c: char) {
        self.chars.push(c);
    }
}


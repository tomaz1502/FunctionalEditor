use std::fs;

pub struct Config {
    pub file: Option<String>,
    pub text: Option<String>,
    max_col: u16,
    max_row: u16,
    min_col: u16,
    min_row: u16,
}

impl Config {
    pub fn new(args: &Vec<String>, max_row: u16, max_col: u16) -> Result<Config, &'static str> {
        if args.len() > 3 {
            return Err("Too many arguments! Usage: cargo run file_name");
        }

        if args.len() == 2 {
            let file_name = args[1].clone();
            let file_text = fs::read_to_string(&file_name);
            match file_text {
                Ok(ft) => Ok(Config {
                    file: Some(file_name),
                    text: Some(ft),
                    max_col,
                    max_row,
                    min_col: 4,
                    min_row: 1,
                }),
                Err(_) => Err("File not found!"),
            }
        } else {
            Ok(Config {
                file: None,
                text: None,
                max_row,
                max_col,
                min_col: 4,
                min_row: 1,
            })
        }
    }

    pub fn max_row(&self) -> u16 {
        self.max_row
    }

    pub fn max_col(&self) -> u16 {
        self.max_col
    }
    
    pub fn min_col(&self) -> u16 {
        self.min_col
    }

    pub fn min_row(&self) -> u16 {
        self.min_row
    }
}

use std::fs;

pub struct Config {
    pub file: Option<String>,
    pub text: Option<String>,
    pub log: fs::File,
    max_col: u16,
    max_row: u16,
    left_most: u16,
}

impl Config {
    pub fn new(args: &Vec<String>, max_row: u16, max_col: u16) -> Result<Config, &'static str> {

        if args.len() > 3 {
            return Err("Too many arguments! Usage: cargo run file_name");
        }

        if args.len() > 1 {
            let file_name = args[1].clone();
            let file_text = fs::read_to_string(&file_name);
            match file_text {
                Ok(ft)  => Ok(Config {
                                       file: Some(file_name),
                                       text: Some(ft),
                                       log: fs::File::create("log").unwrap(),
                                       max_col,
                                       max_row,
                                       left_most: 4
                                     }),
                Err(_) => Err("File not found!") 
            }
        }
        else {
            Ok(Config {
                        file: None,
                        text: None,
                        log: fs::File::create("log").unwrap(),
                        max_row,
                        max_col,
                        left_most: 4,
                      })
        }
    }

    pub fn left_most(&self) -> u16 {
        self.left_most
    }

    pub fn max_row(&self) -> u16 {
        self.max_row
    }

    pub fn max_col(&self) -> u16 {
        self.max_col
    }
}

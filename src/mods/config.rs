use std::fs;

pub struct Config {
    pub file: Option<String>,
    pub text: Option<String>,
    max_col: usize,
    max_row: usize,
    left_most: usize,
}

impl Config {
    pub fn new(args: &Vec<String>, max_row: usize, max_col: usize) -> Result<Config, &'static str> {

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
                        max_row,
                        max_col,
                        left_most: 4,
                      })
        }
    }

    pub fn left_most(&self) -> usize {
        self.left_most
    }

    pub fn max_row(&self) -> usize {
        self.max_row
    }

    pub fn max_col(&self) -> usize {
        self.max_col
    }
}

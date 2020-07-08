use std::fs;

pub struct Config {
    pub file: Option<String>,
    pub text: Option<String>,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {

        if args.len() > 3 {
            return Err("Too many arguments! Usage: cargo run file_name");
        }



        if args.len() > 1 {
            let file_name = args[1].clone();
            let file_text = fs::read_to_string(&file_name);
            match file_text {
                Ok(ft)  => Ok(Config { file: Some(file_name), text: Some(ft), }),
                Err(_) => Err("File not found!") 
            }
        }
        else {
            Ok(Config { file: None, text: None, })
        }
    }
}

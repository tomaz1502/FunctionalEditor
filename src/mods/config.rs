use std::fs::File;
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub file: Option<File>,
    pub cwd: PathBuf,
    width: u16,
    height: u16,
    min_col: u16,
    min_row: u16,
}

impl Config {
    pub fn new(args: &Vec<String>, height: u16, width: u16) -> Result<Config, &'static str> {
        if args.len() >= 3 {
            return Err("Too many arguments! Usage: cargo run <file_name>");
        }

        let cwd = env::current_dir().unwrap();
        let file = match args.len() {
            2 => { let file_name = args[1].clone();
                   match File::open(&file_name) {
                     Ok(file) => Some(file),
                     Err(_)   => Some(File::create(&file_name).unwrap()),
                   }
                 },
            _ => None,
        };
        Ok(Config {
            file,
            cwd,
            width,
            height,
            min_col: 4,
            min_row: 1,
        })
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
    
    pub fn min_col(&self) -> u16 {
        self.min_col
    }

    pub fn min_row(&self) -> u16 {
        self.min_row
    }
}

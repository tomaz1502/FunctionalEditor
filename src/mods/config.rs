pub struct Config {
    pub file_name : Option<String>,
    width         : u16,
    height        : u16,
    min_col       : u16,
    min_row       : u16,
}

impl Config {
    pub fn new(args: &Vec<String>, height: u16, width: u16) -> Result<Config, &'static str> {
        if args.len() >= 3 {
            return Err("Too many arguments! Usage: cargo run <file_name>");
        }

        let file_name = match args.len() {
            2 => Some(args[1].clone()),
            _ => None
        };

        Ok(Config {
            file_name,
            width,
            height: height - 2,
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

use super::lib::get_extension;

use super::languages::ColorsConfig;
use super::languages::haskell::HaskellConfig;

pub struct Config {
    pub file_name : Option<String>,
    width         : u16,
    height        : u16,
    min_col       : u16,
    min_row       : u16,
    colors_cfg    : ColorsConfig,
}

impl Config {
    pub fn new(args: &Vec<String>, height: u16, width: u16)
              -> Result<Config, &'static str> {
        if args.len() >= 3 {
            return Err("Too many arguments! Usage: cargo run <file_name>");
        }

        let file_name = match args.len() {
            2 => Some(args[1].clone()),
            _ => None
        };

        let colors_cfg = HaskellConfig;

        Ok(Config {
            file_name,
            width,
            height: height - 2,
            min_col: 4,
            min_row: 1,
            colors_cfg,
        })
    }

    pub fn set_filename(&mut self, name: &String) {
        self.file_name = Some(name.clone());
        self.colors_cfg = match get_extension(name) {
            Some(ext) => match &ext[..] {
                            "hs" => HaskellConfig,
                            _    => HaskellConfig,
                         },
            None    => HaskellConfig,
        };
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

    pub fn color_from_word(&self, word: &String) -> &'static str {
        let all_digits = word.chars().all(|c| c.is_digit(10));
        if all_digits {
            self.colors_cfg.num_color
        }
        else if (self.colors_cfg.is_keyword)(word) {
            self.colors_cfg.keyword_color
        } else if (self.colors_cfg.is_type_name)(word) {
            self.colors_cfg.type_name_color
        } else {
            self.colors_cfg.default_color
        }
    }
}

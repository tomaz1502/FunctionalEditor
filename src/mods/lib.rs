use super::languages::{ ColorsConfig,
                        haskell::HaskellConfig,
                        rust::RustConfig };

pub fn get_color_config(from: &String) -> ColorsConfig {
    match get_extension(from) {
        Some(ext) => match &ext[..] {
                        "hs" => HaskellConfig,
                        "rs" => RustConfig,
                         _   => Default::default(),
                     },
        None      => Default::default(),
    }
}

pub fn get_extension(file_name: &String) -> Option<String> {
    let words = file_name.split(".")
                         .map(|word| word.to_string())
                         .collect::<Vec<String>>();
    if words.len() < 2 {
        None
    } else {
        words.last().cloned()
    }
}

pub fn is_separator(ch: char) -> bool {
    ch == ' ' || ch == '.' || ch == '(' || ch == ')'
}

pub fn words_and_separators(text: &str) -> Vec<(String, String)> {
    let mut words_and_separators = Vec::new();
    let mut chunk = String::new();
    let mut separators = String::new();
    for ch in text.chars() {
        if is_separator(ch) {
            separators.push(ch);
        } else {
            if separators.is_empty() {
                chunk.push(ch);
            } else {
                words_and_separators.push((chunk.clone(), separators.clone()));
                chunk = String::from(ch);
                separators.clear();
            }
        }
    }
    words_and_separators.push((chunk.clone(), separators.clone()));
    words_and_separators
}

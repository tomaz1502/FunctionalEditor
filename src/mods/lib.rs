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

pub fn words_and_spaces(text: &str) -> Vec<(String, String)> {
    let mut words_and_spaces = Vec::new();
    let mut chunk = String::new();
    let mut whitespaces = String::new();
    for ch in text.chars() {
        if ch.is_whitespace() {
            whitespaces.push(ch);
        } else {
            if whitespaces.is_empty() {
                chunk.push(ch);
            } else {
                words_and_spaces.push((chunk.clone(), whitespaces.clone()));
                chunk = String::from(ch);
                whitespaces.clear();
            }
        }
    }
    words_and_spaces.push((chunk.clone(), whitespaces.clone()));
    words_and_spaces
}

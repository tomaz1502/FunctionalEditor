#[derive(Clone)]
pub struct Data {
    info: Vec<String>,
}

impl Data {
    pub fn from_vec(source: Vec<String>) -> Data {
        Data { info: source }
    }
    pub fn len(&self) -> usize {
        self.info.len()
    }
    pub fn row_length(&self, row: u16) -> usize {
        self.info[row as usize].len()
    }
    pub fn get_row_mut(&mut self, row: u16) -> &mut String {
        &mut self.info[row as usize]
    }
    pub fn get_row(&self, row: u16) -> &String {
        &self.info[row as usize]
    }
    pub fn remove(&mut self, row: u16) {
        self.info.remove(row as usize);
    }
    pub fn remove_char(&mut self, row: u16, col: u16) {
        self.info[row as usize].remove(col as usize);
    }
    pub fn insert(&mut self, row: u16, text: String) {
        self.info.insert(row as usize, text);
    }
    pub fn insert_char(&mut self, row: u16, col: u16, c: char) {
        self.info[row as usize].insert(col as usize, c);
    }
    pub fn truncate_row(&mut self, row: u16, trunc_pos: u16) {
        self.info[row as usize].drain(trunc_pos as usize ..);
    }
    pub fn extend_row(&mut self, row: u16, text: String) {
        self.info[row as usize].extend(text.chars());
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        self.info.clone().join("\n") + "\n"
    }
}

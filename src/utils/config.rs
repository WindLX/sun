#[derive(Debug)]
pub struct Config {
    pub check_tokenizer: bool,
    pub check_parser: bool,
    pub check_command: bool,
    pub is_debug: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            check_tokenizer: false,
            check_parser: false,
            check_command: false,
            is_debug: false,
        }
    }
}
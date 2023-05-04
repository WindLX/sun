#[derive(Debug)]
pub struct Config {
    pub check_tokenizer: bool,
    pub check_parser: bool,
    pub check_command: bool,
    pub check_stack: bool,
    pub check_global: bool,
    pub is_debug: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            check_tokenizer: false,
            check_parser: false,
            check_command: false,
            check_stack: false,
            check_global: false,
            is_debug: false,
        }
    }
}

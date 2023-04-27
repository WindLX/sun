use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SunError {
    TokenizerError(String, u64),
    InvalidNumberError(String, u64),
    InvalidSymbolError(String, u64),
    CallError(String, u64),
}

impl fmt::Display for SunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerError(s, l) => write!(f, "TokenizerError: {} at line {l}", s),
            Self::InvalidNumberError(s, l) => write!(f, "InvalidNumberError: {} at line {l}", s),
            Self::InvalidSymbolError(s, l) => write!(f, "InvalidSymbolError: {} at line {l}", s),
            Self::CallError(s, l) => write!(f, "CallError: {} at call stack index of {l}", s),
        }
    }
}

impl Error for SunError {}

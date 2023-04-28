use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SunError {
    TokenizerError(String, u64),
    NumberError(String, u64),
    SymbolError(String, u64),
    CallError(String, u64),
    ParaError(String, u64),
    InputError(String),
}

impl fmt::Display for SunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerError(s, l) => write!(f, "  TokenizerError: {} at line {l}", s),
            Self::NumberError(s, l) => write!(f, "  NumberError: {} at line {l}", s),
            Self::SymbolError(s, l) => write!(f, "  SymbolError: {} at line {l}", s),
            Self::CallError(s, l) => write!(f, "  CallError: {} at call stack index of {l}", s),
            Self::ParaError(s, l) => write!(f, "  ParaError: {} at line {l}", s),
            Self::InputError(s) => write!(f, "  InputError: {s}"),
        }
    }
}

impl Error for SunError {}

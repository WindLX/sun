use colorized::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SunError {
    TokenizerError(String, u64),
    NumberError(String, u64),
    SymbolError(String, u64),
    CallError(String),
    ParaError(String, u64),
    AssignError(String, u64),
    KeyError(String),
    IndexError(String),
    TypeError(String),
    InputError(String),
}

impl fmt::Display for SunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerError(s, l) => write!(
                f,
                "{}: {s} at line {l}",
                "TokenizerError".color(Colors::RedFg)
            ),
            Self::NumberError(s, l) => {
                write!(f, "{}: {s} at line {l}", "NumberError".color(Colors::RedFg))
            }
            Self::SymbolError(s, l) => {
                write!(f, "{}: {s} at line {l}", "SymbolError".color(Colors::RedFg))
            }
            Self::CallError(s) => write!(f, "{}: {s}", "CallError".color(Colors::RedFg)),
            Self::ParaError(s, l) => {
                write!(f, "{}: {s} at line {l}", "ParaError".color(Colors::RedFg))
            }
            Self::AssignError(s, l) => {
                write!(f, "{}: {s} at line {l}", "AssignError".color(Colors::RedFg))
            }
            Self::KeyError(s) => {
                write!(f, "{}: {s}", "KeyError".color(Colors::RedFg))
            }
            Self::IndexError(s) => {
                write!(f, "{}: {s}", "IndexError".color(Colors::RedFg))
            }
            Self::TypeError(s) => write!(f, "{}: {s}", "TypeError".color(Colors::RedFg)),
            Self::InputError(s) => write!(f, "{}: {s}", "InputError".color(Colors::RedFg)),
        }
    }
}

impl Error for SunError {}

use colorized::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SunError {
    TokenizerError(String),
    NumberError(String),
    SymbolError(String),
    CallError(String),
    ParaError(String),
    AssignError(String),
    KeyError(String),
    IndexError(String),
    RunError(String),
    TypeError(String),
    AttributeError(String),
    InputError(String),
}

impl fmt::Display for SunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerError(s) => write!(f, "{} ({s})", "TokenizerError".color(Colors::RedFg)),
            Self::NumberError(s) => {
                write!(f, "{} ({s})", "NumberError".color(Colors::RedFg))
            }
            Self::SymbolError(s) => {
                write!(f, "{} ({s})", "SymbolError".color(Colors::RedFg))
            }
            Self::CallError(s) => write!(f, "{} ({s})", "CallError".color(Colors::RedFg)),
            Self::ParaError(s) => {
                write!(f, "{} ({s})", "ParaError".color(Colors::RedFg))
            }
            Self::AssignError(s) => {
                write!(f, "{} ({s})", "AssignError".color(Colors::RedFg))
            }
            Self::KeyError(s) => {
                write!(f, "{} ({s})", "KeyError".color(Colors::RedFg))
            }
            Self::IndexError(s) => {
                write!(f, "{} ({s})", "IndexError".color(Colors::RedFg))
            }
            Self::RunError(s) => {
                write!(f, "{} ({s})", "RunError".color(Colors::RedFg))
            }
            Self::TypeError(s) => write!(f, "{} ({s})", "TypeError".color(Colors::RedFg)),
            Self::AttributeError(s) => write!(f, "{} ({s})", "AttributeError".color(Colors::RedFg)),
            Self::InputError(s) => write!(f, "{} ({s})", "InputError".color(Colors::RedFg)),
        }
    }
}

impl Error for SunError {}

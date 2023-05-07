use colorized::*;
use std::error::Error;
use std::fmt;

/// Sun 的错误类型
#[derive(Debug, PartialEq)]
pub enum SunError {
    /// `TokenizerError`: 词法分析阶段的错误
    TokenizerError(String),
    /// `NumberError`: 非法数字错误，词法分析阶段
    NumberError(String),
    /// `SymbolError`: 非法符号错误，词法和语法分析阶段
    SymbolError(String),
    /// `CallError`: 函数调用错误，虚拟机指令执行阶段
    CallError(String),
    /// `ParaError`: 函数参数错误，数量错误或类型错误，给函数传参阶段
    ParaError(String),
    /// `AssignError`: 赋值语句错误，无效的赋值语句，语法分析阶段
    AssignError(String),
    /// `KeyError`: 键名错误，使用键对table进行处理时触发，非法键名或无效键名
    KeyError(String),
    /// `IndexError`: 索引错误，使用索引对table进行处理时触发，非法索引或无效索引
    IndexError(String),
    /// `RunError`: 虚拟机执行期间的错误，通常原因可能是虚拟机栈为空时取值
    RunError(String),
    /// `TypeError`: 向meta表中查找一个非法类型时触发
    TypeError(String),
    /// `AttributeError`: 向meta表指定类型下查找attribute失败或者语法分析阶段处理一个非法的dot表达式时触发
    AttributeError(String),
    /// `InputError`: 输入错误，解释器启动阶段触发，接收到错误的sun文件或者命令行输入时触发
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

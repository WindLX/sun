use crate::{
    container::{Class, Function, RustFunction, SysFunction, Table},
    utils::{log::error_output, SunError},
};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;

/// 类型数据的容器
#[derive(Clone)]
pub enum SunValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(Vec<u8>),
    Table(Table),
    Function(Function),
    Class(Class),
}

impl SunValue {
    pub fn get_name(&self) -> &str {
        match self {
            SunValue::Nil => "Nil",
            SunValue::Boolean(_) => "Bool",
            SunValue::Number(_) => "Number",
            SunValue::String(_) => "String",
            SunValue::Table(_) => "Table",
            SunValue::Function(_) => "Function",
            SunValue::Class(c) => c.get_name(),
        }
    }
}

impl From<()> for SunValue {
    fn from(_: ()) -> Self {
        SunValue::Nil
    }
}

impl From<bool> for SunValue {
    fn from(value: bool) -> Self {
        SunValue::Boolean(value)
    }
}

impl From<f64> for SunValue {
    fn from(value: f64) -> Self {
        SunValue::Number(value)
    }
}

impl From<Vec<u8>> for SunValue {
    fn from(value: Vec<u8>) -> Self {
        SunValue::String(value)
    }
}

impl From<String> for SunValue {
    fn from(value: String) -> Self {
        SunValue::String(value.into_bytes())
    }
}

impl From<&str> for SunValue {
    fn from(value: &str) -> Self {
        SunValue::String(value.as_bytes().into())
    }
}

impl From<&[u8]> for SunValue {
    fn from(value: &[u8]) -> Self {
        SunValue::String(value.to_vec())
    }
}

impl<'a> From<&'a SunValue> for &'a [u8] {
    fn from(value: &'a SunValue) -> Self {
        if let SunValue::String(s) = value {
            s
        } else {
            let e = SunError::ParaError(format!("invalid string"));
            error_output(e);
        }
    }
}

impl<'a> From<&'a SunValue> for Cow<'a, str> {
    fn from(value: &'a SunValue) -> Self {
        if let SunValue::String(s) = value {
            Cow::from(String::from_utf8_lossy(s.as_slice()))
        } else {
            let e = SunError::ParaError(format!("invalid string"));
            error_output(e);
        }
    }
}

impl From<&SunValue> for String {
    fn from(value: &SunValue) -> Self {
        String::from_utf8_lossy(value.into()).to_string()
    }
}

impl From<Table> for SunValue {
    fn from(value: Table) -> Self {
        SunValue::Table(value)
    }
}

impl From<Function> for SunValue {
    fn from(value: Function) -> Self {
        SunValue::Function(value)
    }
}

impl From<RustFunction> for SunValue {
    fn from(value: RustFunction) -> Self {
        SunValue::Function(value.into())
    }
}

impl From<SysFunction> for SunValue {
    fn from(value: SysFunction) -> Self {
        SunValue::Function(value.into())
    }
}

impl From<Class> for SunValue {
    fn from(value: Class) -> Self {
        SunValue::Class(value)
    }
}

impl fmt::Display for SunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunValue::Nil => write!(f, "nil"),
            SunValue::Boolean(b) => write!(f, "{}", b),
            SunValue::Number(n) => write!(f, "{}", n),
            SunValue::String(s) => {
                write!(
                    f,
                    "{}",
                    Cow::from(String::from_utf8_lossy(s.as_slice())).to_string()
                )
            }
            SunValue::Table(t) => write!(f, "{}", t),
            SunValue::Function(p) => write!(f, "{}", p),
            SunValue::Class(c) => write!(f, "{}", c),
        }
    }
}

impl fmt::Debug for SunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunValue::Nil => write!(f, "Nil"),
            SunValue::Boolean(b) => write!(f, "Bool({})", b),
            SunValue::Number(n) => write!(f, "Number({})", n),
            SunValue::String(s) => {
                write!(
                    f,
                    "String({})",
                    Cow::from(String::from_utf8_lossy(s.as_slice())).to_string()
                )
            }
            SunValue::Table(t) => write!(f, "Table({})", t),
            SunValue::Function(p) => write!(f, "Function({})", p),
            SunValue::Class(c) => write!(f, "Class({})", c.get_name()),
        }
    }
}

impl PartialEq for SunValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SunValue::Boolean(x), SunValue::Boolean(y)) => x == y,
            (SunValue::Number(x), SunValue::Number(y)) => x == y,
            (SunValue::String(x), SunValue::String(y)) => x == y,
            (SunValue::Table(x), SunValue::Table(y)) => x == y,
            _ => false,
        }
    }
}

impl Eq for SunValue {}

impl PartialOrd for SunValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (SunValue::Boolean(x), SunValue::Boolean(y)) => Some(x.cmp(y)),
            (SunValue::Number(x), SunValue::Number(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

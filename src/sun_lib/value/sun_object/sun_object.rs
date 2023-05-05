use super::super::{
    sun_function::Function,
    sun_meta::{OwnSunMeta, SunMeta},
    sun_pointer::SunPointer,
    sun_table::Table,
};
use crate::utils::{err::SunError, log::error_output};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
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
}

impl SunValue {
    pub fn get_name(&self) -> &str {
        match self {
            SunValue::Nil => "nil",
            SunValue::Boolean(_) => "bool",
            SunValue::Number(_) => "number",
            SunValue::String(_) => "string",
            SunValue::Table(_) => "table",
            SunValue::Function(_) => "function",
        }
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
            SunValue::Function(p) => write!(f, "<function: {:p}>", p),
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
            SunValue::Function(p) => write!(f, "Function({:p})", p),
        }
    }
}

/// `Object` 元数据
#[derive(Debug, Clone)]
pub struct SunObject {
    meta: SunMeta,
}

impl SunObject {
    /// 新建新的 `Object` 元数据
    pub fn new(name: &'static str) -> SunObject {
        let meta = SunMeta::new(name, HashMap::new());
        let mut obj = SunObject { meta };
        obj.set_method("type", _type());
        obj.set_method("clone", clone());
        obj
    }
}

/// 获取类型名的类型方法
pub fn _type() -> Function {
    let f = |value: Vec<SunPointer>| -> Vec<SunPointer> {
        let value = value[0].get();
        let res = SunPointer::new(SunValue::from(value.get_name().to_string()));
        vec![res]
    };
    f
}

// 获取数据的拷贝
pub fn clone() -> Function {
    let f = |value: Vec<SunPointer>| -> Vec<SunPointer> {
        let res = value[0].deep_copy();
        vec![res]
    };
    f
}

/// 继承自 Object 的特征
pub trait IsSunObject {
    /// 有能力生成一个 Object
    fn get_obj(&self) -> SunObject;
}

impl OwnSunMeta for SunObject {
    fn get_method(&self, key: &str) -> Option<Function> {
        self.meta.get_method(key)
    }

    fn set_method(&mut self, key: &str, value: Function) {
        self.meta.set_method(key, value)
    }
}

/// 批量添加类型方法
#[macro_export]
macro_rules! add_methods {
    ($obj:expr, $type_name:ty, $(($name:expr, $method:ident)),+) => {
        $(
            $obj.set_method($name, <$type_name>::$method());
        )+
    };
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

impl PartialEq for SunValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SunValue::Boolean(x), SunValue::Boolean(y)) => x == y,
            (SunValue::Number(x), SunValue::Number(y)) => x == y,
            (SunValue::String(x), SunValue::String(y)) => x == y,
            (SunValue::Table(x), SunValue::Table(y)) => x == y,
            (SunValue::Function(x), SunValue::Function(y)) => x == y,
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

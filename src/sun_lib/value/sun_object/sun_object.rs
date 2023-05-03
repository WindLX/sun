use super::super::{
    sun_function::Function,
    sun_meta::{OwnSunMeta, SunMeta},
    sun_pointer::SunPointer,
    sun_table::Table,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SunValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
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
            SunValue::String(s) => write!(f, "{}", s),
            SunValue::Table(t) => write!(f, "{}", t),
            SunValue::Function(p) => write!(f, "<function: {:p}>", p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SunObject {
    meta: SunMeta,
}

impl SunObject {
    pub fn new(name: &'static str) -> SunObject {
        let meta = SunMeta::new(name, HashMap::new());
        let mut obj = SunObject { meta };
        obj.set_method("type", _type());
        obj
    }
}

pub fn _type() -> Function {
    let f = |value: Vec<SunPointer>| -> Vec<SunPointer> {
        let value = value[0].get();
        let res = SunPointer::new(SunValue::from(value.get_name().to_string()));
        vec![res]
    };
    f
}

pub trait IsSunObject {
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

impl From<String> for SunValue {
    fn from(value: String) -> Self {
        SunValue::String(value)
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

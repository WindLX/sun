use super::SunValue;
use std::fmt;

#[derive(Clone)]
pub enum SunType {
    Nil,
    Boolean,
    Number,
    Complex,
    String,
    Table,
    Function,
}

impl PartialEq for SunType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SunType::Nil, SunType::Nil) => true,
            (SunType::Boolean, SunType::Boolean) => true,
            (SunType::Number, SunType::Number) => true,
            (SunType::Function, SunType::Function) => true,
            (SunType::Complex, SunType::Complex) => true,
            (SunType::String, SunType::String) => true,
            (SunType::Table, SunType::Table) => true,
            (_, _) => false,
        }
    }
}

impl Eq for SunType {}

impl fmt::Debug for SunType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunType::Nil => write!(f, "type: Nil"),
            SunType::Boolean => write!(f, "type: Boolean"),
            SunType::Number => write!(f, "type: Number"),
            SunType::Complex => write!(f, "type: Complex"),
            SunType::Function => write!(f, "type: Function"),
            SunType::String => write!(f, "type: String"),
            SunType::Table => write!(f, "type: Table"),
        }
    }
}

impl fmt::Display for SunType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunType::Nil => write!(f, "type: Nil"),
            SunType::Boolean => write!(f, "type: Boolean"),
            SunType::Number => write!(f, "type: Number"),
            SunType::Complex => write!(f, "type: Complex"),
            SunType::Function => write!(f, "type: Function"),
            SunType::String => write!(f, "type: String"),
            SunType::Table => write!(f, "type: Table"),
        }
    }
}

impl From<SunValue> for SunType {
    fn from(value: SunValue) -> Self {
        match value {
            SunValue::Nil => SunType::Nil,
            SunValue::Boolean(_) => SunType::Boolean,
            SunValue::Function(_) => SunType::Function,
            SunValue::Number(_) => SunType::Function,
            SunValue::Complex(_, _) => SunType::Complex,
            SunValue::String(_) => SunType::String,
            SunValue::Table(_) => SunType::Table,
            SunValue::Pointer(p) => p.get_type(),
        }
    }
}

use crate::vm::machine::VirtualMachine;
use std::fmt;
// use std::hash::Hash;
// use std::mem;
// use std::rc::Rc;

#[derive(Clone)]
pub enum SunValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Complex(f64, f64),
    Tensor(Vec<SunValue>),
    Function(SunFunc),
}

pub type SunFunc = fn(&mut VirtualMachine);

impl fmt::Debug for SunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunValue::Nil => write!(f, "<nil>"),
            SunValue::Boolean(b) => write!(f, "<bool: {b}>"),
            SunValue::Number(n) => write!(f, "<number: {n:?}>"),
            SunValue::Complex(r, i) => write!(f, "<complex: {r:?}+{i:?}i>"),
            SunValue::Function(_) => write!(f, "<function>"),
            SunValue::Tensor(t) => write!(f, "<tensor: {:?}>", t),
        }
    }
}

impl fmt::Display for SunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunValue::Nil => write!(f, "nil"),
            SunValue::Boolean(b) => write!(f, "{b}"),
            SunValue::Number(n) => write!(f, "{n:?}"),
            SunValue::Complex(r, i) => write!(f, "{r:?}+{i:?}i"),
            SunValue::Function(_) => write!(f, "function"),
            SunValue::Tensor(t) => write!(f, "tensor: {:?}", t),
        }
    }
}

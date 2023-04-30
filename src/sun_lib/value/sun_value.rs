use crate::sun_lib::{pointer::SunPointer, table::Table};
use crate::vm::machine::VirtualMachine;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;
use std::rc::Rc;

#[derive(Clone)]
pub enum SunValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Complex(f64, f64),
    String(Rc<String>),
    Table(Table),
    Pointer(SunPointer),
    Function(SunFunc),
}

pub type SunFunc = fn(&mut VirtualMachine) -> u8;

impl fmt::Debug for SunValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunValue::Nil => write!(f, "<nil>"),
            SunValue::Boolean(b) => write!(f, "<bool: {b}>"),
            SunValue::Number(n) => write!(f, "<number: {n}>"),
            SunValue::String(s) => write!(f, "<string: {s}>"),
            SunValue::Complex(r, i) => write!(f, "<complex: {r}+{i}i>"),
            SunValue::Function(_) => write!(f, "<function>"),
            SunValue::Table(t) => write!(f, "{:?}", t),
            SunValue::Pointer(p) => write!(f, "{:?}", p.clone()),
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
            SunValue::String(s) => write!(f, "{s}"),
            SunValue::Function(ff) => write!(f, "function: {:p}", *ff as *const u64),
            SunValue::Table(t) => write!(f, "{}", t),
            SunValue::Pointer(p) => write!(f, "{}", p.clone()),
        }
    }
}

impl PartialEq for SunValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SunValue::Nil, SunValue::Nil) => true,
            (SunValue::Boolean(b1), SunValue::Boolean(b2)) => *b1 == *b2,
            (SunValue::Number(n1), SunValue::Number(n2)) => *n1 == *n2,
            (SunValue::Complex(r1, i1), SunValue::Complex(r2, i2)) => *r1 == *r2 && *i1 == *i2,
            (SunValue::String(s1), SunValue::String(s2)) => *s1 == *s2,
            (SunValue::Table(t1), SunValue::Table(t2)) => *t1 == *t2,
            (SunValue::Function(f1), SunValue::Function(f2)) => ptr::eq(f1, f2),
            (SunValue::Pointer(p1), SunValue::Pointer(p2)) => p1 == p2,
            (_, _) => false,
        }
    }
}

impl Eq for SunValue {}

impl Hash for SunValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SunValue::Nil => ().hash(state),
            SunValue::Boolean(b) => b.hash(state),
            SunValue::Number(f) => unsafe { mem::transmute::<f64, i64>(*f).hash(state) },
            SunValue::Complex(r, i) => unsafe { mem::transmute::<f64, i64>(*r + *i).hash(state) },
            SunValue::String(s) => s.hash(state),
            SunValue::Table(t) => t.hash(state),
            SunValue::Pointer(p) => p.hash(state),
            SunValue::Function(f) => (*f as *const u64).hash(state),
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

impl From<(f64, f64)> for SunValue {
    fn from(value: (f64, f64)) -> Self {
        SunValue::Complex(value.0, value.1)
    }
}

impl From<[f64; 2]> for SunValue {
    fn from(value: [f64; 2]) -> Self {
        SunValue::Complex(value[0], value[1])
    }
}

impl From<&str> for SunValue {
    fn from(value: &str) -> Self {
        SunValue::String(Rc::new(value.to_string()))
    }
}

impl From<String> for SunValue {
    fn from(value: String) -> Self {
        SunValue::String(Rc::new(value))
    }
}

impl From<fn(&mut VirtualMachine) -> u8> for SunValue {
    fn from(value: fn(&mut VirtualMachine) -> u8) -> Self {
        SunValue::Function(value)
    }
}

impl From<Table> for SunValue {
    fn from(value: Table) -> Self {
        SunValue::Table(value)
    }
}

impl From<SunPointer> for SunValue {
    fn from(value: SunPointer) -> Self {
        SunValue::Pointer(value.clone())
    }
}

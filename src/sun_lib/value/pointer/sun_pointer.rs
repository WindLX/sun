use crate::sun_lib::value::{SunType, SunValue};
use crate::utils::err::SunError;
use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct SunPointer {
    sun_type: RefCell<SunType>,
    pointer: Rc<RefCell<SunValue>>,
}

impl SunPointer {
    pub fn new(value: SunValue) -> Self {
        SunPointer {
            sun_type: RefCell::new(SunType::from(value.clone())),
            pointer: Rc::new(RefCell::new(value)),
        }
    }

    pub fn set_value(&self, value: SunValue) {
        *self.sun_type.borrow_mut() = SunType::from(value.clone());
        *self.pointer.borrow_mut() = value;
    }

    pub fn get_type(&self) -> SunType {
        self.sun_type.borrow().clone()
    }

    pub fn get_by_key(&self, key: &str) -> Option<SunPointer> {
        let p = self.pointer.clone();
        let pp = p.borrow_mut();
        match pp.clone() {
            SunValue::Table(tt) => tt.get_by_key(key),
            _ => None,
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<SunPointer> {
        let p = self.pointer.clone();
        let pp = p.borrow_mut();
        match pp.clone() {
            SunValue::Table(tt) => tt.get_by_index(index),
            _ => None,
        }
    }

    pub fn push_by_index(&self, value: SunValue) -> Result<(), SunError> {
        let mut p = self.pointer.borrow_mut();
        if let SunValue::Table(table) = &mut *p {
            table.push(value);
            Ok(())
        } else {
            Err(SunError::TypeError(format!(
                "expect `table` but got `{}`",
                self.get_type()
            )))
        }
    }

    pub fn insert_by_kv(&self, key: String, value: SunValue) -> Result<(), SunError> {
        let mut p = self.pointer.borrow_mut();
        if let SunValue::Table(table) = &mut *p {
            table.insert_kv(key, value);
            Ok(())
        } else {
            Err(SunError::TypeError(format!(
                "expect `table` but got `{}`",
                self.get_type()
            )))
        }
    }

    pub fn remove_by_key(&self, key: String) -> Result<SunValue, SunError> {
        let mut p = self.pointer.borrow_mut();
        if let SunValue::Table(table) = &mut *p {
            match table.remove_by_key(key.clone()) {
                Some(v) => Ok(v),
                None => Err(SunError::KeyError(format!(
                    "failed to find target value by key `{}`",
                    key
                ))),
            }
        } else {
            Err(SunError::TypeError(format!(
                "expect `table` but got `{}`",
                self.get_type()
            )))
        }
    }

    pub fn remove_by_index(&self, index: usize) -> Result<SunValue, SunError> {
        let mut p = self.pointer.borrow_mut();
        if let SunValue::Table(table) = &mut *p {
            match table.remove_by_index(index) {
                Some(v) => Ok(v),
                None => Err(SunError::IndexError(format!(
                    "index `{index}` out of range"
                ))),
            }
        } else {
            Err(SunError::TypeError(format!(
                "expect `table` but got `{}`",
                self.get_type()
            )))
        }
    }

    pub fn get_func(&self) -> Option<SunValue> {
        let f = self.pointer.borrow().clone();
        match f {
            SunValue::Function(_) => Some(f),
            _ => None,
        }
    }

    pub fn as_ptr(&self) -> *const SunValue {
        self.pointer.as_ptr()
    }

    pub fn get_content(&self) -> SunValue {
        let p = self.pointer.borrow();
        p.clone()
    }
}

impl Clone for SunPointer {
    fn clone(&self) -> Self {
        let sun_type = self.sun_type.clone();
        Self {
            sun_type,
            pointer: self.pointer.clone(),
        }
    }
}

impl Hash for SunPointer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl PartialEq for SunPointer {
    fn eq(&self, other: &Self) -> bool {
        if self.sun_type != other.sun_type {
            return false;
        }
        if self.as_ptr() != other.as_ptr() {
            return false;
        }
        true
    }
}

impl fmt::Debug for SunPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}>", self.pointer.borrow())
    }
}

impl fmt::Display for SunPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pointer.borrow())
    }
}

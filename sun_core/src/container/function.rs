use crate::{
    sunc::sun_struct::{FunctionC, FunctionType},
    utils::{IsMachine, SunPointer},
};
use std::fmt;

/// Function 的数据
#[derive(Clone)]
pub enum Function {
    RustFunction(RustFunction),
    SysFunction(SysFunction),
}

impl From<RustFunction> for Function {
    fn from(value: RustFunction) -> Self {
        Function::RustFunction(value)
    }
}

impl From<SysFunction> for Function {
    fn from(value: SysFunction) -> Self {
        Function::SysFunction(value)
    }
}

impl From<FunctionC> for Function {
    fn from(value: FunctionC) -> Self {
        match value._type {
            FunctionType::RustFunction => {
                Function::from(unsafe { *(value.data.rust_function as *const RustFunction) })
            }
            FunctionType::SysFunction => {
                Function::from(unsafe { *(value.data.sys_function as *const SysFunction) })
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function")
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// RustFunction 的数据
pub type RustFunction = fn(Vec<SunPointer>) -> Vec<SunPointer>;

/// SysFunction 的数据
pub type SysFunction = fn(&mut dyn IsMachine);

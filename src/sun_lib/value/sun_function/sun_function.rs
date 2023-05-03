use super::super::{
    sun_meta::op::CallAble,
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::{add_methods, sun_lib::value::sun_meta::OwnSunMeta};
use std::fmt;

pub type Function = fn(Vec<SunPointer>) -> Vec<SunPointer>;

#[derive(Clone)]
pub struct SunFunction {
    obj: SunObject,
}

impl SunFunction {
    pub fn new() -> Self {
        let mut obj = SunObject::new("function");
        add_methods!(obj, SunFunction, ("call", call));
        SunFunction { obj }
    }

    pub fn as_ptr(&self) -> *const SunFunction {
        self as *const SunFunction
    }
}

impl IsSunObject for SunFunction {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
    }
}

impl CallAble for SunFunction {
    fn call() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let res = match arg {
                SunValue::Function(_) => {
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f
    }
}

impl fmt::Debug for SunFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function: {:p}>", self.as_ptr())
    }
}

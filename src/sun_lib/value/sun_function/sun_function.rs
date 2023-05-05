use super::super::{
    sun_meta::op::CallAble,
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::{add_methods, sun_lib::value::sun_meta::OwnSunMeta};
use std::fmt;

/// Function 的数据(基于Rust)
pub type Function = fn(Vec<SunPointer>) -> Vec<SunPointer>;

/// Function 元数据
#[derive(Clone)]
pub struct SunFunction {
    obj: SunObject,
}

impl SunFunction {
    /// 创建新的 Function 元数据
    pub fn new() -> Self {
        let mut obj = SunObject::new("function");
        add_methods!(obj, SunFunction, ("call", call));
        SunFunction { obj }
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
        write!(f, "<function>")
    }
}

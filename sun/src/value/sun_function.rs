use std::fmt;
use sun_core::utils::{IsSunObject, SunObject};

/// Function 元数据
#[derive(Clone)]
pub struct SunFunction {
    obj: SunObject,
}

impl SunFunction {
    /// 创建新的 Function 元数据
    pub fn new() -> Self {
        let obj = SunObject::new("function");
        SunFunction { obj }
    }
}

impl IsSunObject for SunFunction {
    fn get_obj(&self) -> &SunObject {
        &self.obj
    }

    fn get_mut_obj(&mut self) -> &mut SunObject {
        &mut self.obj
    }
}

impl fmt::Debug for SunFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

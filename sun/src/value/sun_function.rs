use std::fmt;
use sun_core::meta::{OwnSunMeta, SunBase, SunMeta};

/// Function 元数据
#[derive(Clone)]
pub struct SunFunction {
    meta: SunMeta,
}

impl SunFunction {
    /// 创建新的 Function 元数据
    pub fn new() -> Self {
        let meta = SunMeta::new("Function", SunBase::Object);
        SunFunction { meta }
    }
}

impl OwnSunMeta for SunFunction {
    fn get_meta(&self) -> &SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

impl fmt::Debug for SunFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

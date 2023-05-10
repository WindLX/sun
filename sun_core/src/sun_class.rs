use crate::{
    meta::{OwnSunMeta, SunBase, SunMeta},
    utils::{object::SunObject, SunPointer},
};
use std::collections::HashMap;

/// `Class` 类型的元数据
#[derive(Clone)]
pub struct SunClass {
    meta: SunMeta,
}

impl SunClass {
    /// 新建 `Class` 元数据
    pub fn new(class_name: &str, base: SunBase) -> SunClass {
        let meta = SunMeta::new(class_name, base);
        SunClass { meta }
    }
}

impl OwnSunMeta for SunClass {
    fn get_meta(&self) -> &SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

#[derive(Debug)]
pub struct ExportLib {
    pub meta: HashMap<String, SunObject>,
    pub value: HashMap<String, SunPointer>,
}

impl ExportLib {
    pub fn new(meta: HashMap<String, SunObject>, value: HashMap<String, SunPointer>) -> Self {
        ExportLib { meta, value }
    }
}

/// 导出包
pub trait ImportAble {
    /// 返回值为元数据的字典和数据的字典
    fn export_lib() -> ExportLib;
}

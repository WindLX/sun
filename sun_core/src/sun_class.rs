use crate::{
    container::Function,
    meta::OwnSunMeta,
    sunc::tran::ExportLib,
    utils::object::{IsSunObject, SunObject},
};
use std::collections::HashMap;

/// `Class` 类型的元数据
#[derive(Clone)]
pub struct SunClass {
    obj: SunObject,
}

impl SunClass {
    /// 新建 `Class` 元数据
    pub fn new(class_name: &'static str, methods: HashMap<String, Function>) -> SunClass {
        let mut obj = SunObject::new(class_name);
        methods
            .into_iter()
            .for_each(|(name, method)| obj.set_method(name.as_str(), method));
        SunClass { obj }
    }
}

impl IsSunObject for SunClass {
    fn get_obj(&self) -> &SunObject {
        &self.obj
    }

    fn get_mut_obj(&mut self) -> &mut SunObject {
        &mut self.obj
    }
}

/// 导出包
pub trait ImportAble {
    /// 返回值为元数据的字典和数据的字典
    fn export_lib() -> ExportLib;
}

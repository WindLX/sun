use std::collections::HashMap;
use sun_core::{
    container::{Class, IsSunClass, SunValue},
    utils::SunPointer,
};

/// `Complex` 的数据容器
pub struct Complex(Class);

impl Complex {
    /// 新建复数
    pub fn new(i: f64, r: f64) -> Self {
        let mut class = Class::new("complex", HashMap::new());
        class.set_attribute("i", SunPointer::new(SunValue::from(i)));
        class.set_attribute("r", SunPointer::new(SunValue::from(r)));
        Complex(class)
    }
}

impl IsSunClass for Complex {
    fn get_class(&self) -> &Class {
        &self.0
    }
}

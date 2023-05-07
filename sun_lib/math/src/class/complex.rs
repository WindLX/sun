use std::collections::HashMap;
use sun_core::{
    container::{Class, SunValue},
    utils::SunPointer,
};

pub struct Complex(Class);

impl Complex {
    pub fn new(i: f64, r: f64) -> Self {
        let mut class = Class::new("complex", HashMap::new());
        class.set_attribute("i", SunPointer::new(SunValue::from(i)));
        class.set_attribute("r", SunPointer::new(SunValue::from(r)));
        Complex(class)
    }
}

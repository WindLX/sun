pub mod class;
pub mod methods;

use crate::{class::Complex, methods::base::abs};
use sun_core::{
    add_methods,
    meta::{OwnSunMeta, SunBase, SunMeta},
    SunClass,
};

pub struct Math {
    meta: SunMeta,
}

impl Math {
    pub fn new() -> Self {
        let mut class = SunClass::new("Math", SunBase::Object);
        let meta = class.get_meta_mut();
        add_methods!(meta, ("abs", abs));
        Math { meta: meta.clone() }
    }
}

impl OwnSunMeta for Math {
    fn get_meta(&self) -> &sun_core::meta::SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

fn main() {}

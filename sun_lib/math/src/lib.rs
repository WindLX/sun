pub mod class;
pub mod methods;

use crate::{class::Complex, methods::base::abs};
use std::collections::HashMap;
use sun_core::{
    add_type_methods,
    meta::OwnSunMeta,
    sunc::{sun_struct::ExportLibC, tran, tran::ExportLib},
    utils::{IsSunObject, SunObject, SunPointer},
    ImportAble, SunClass,
};

pub struct Math {
    class: SunClass,
}

impl Math {
    pub fn new() -> Self {
        let mut class = SunClass::new("Math", HashMap::new());
        add_type_methods!(class, ("abs", abs));
        Math { class }
    }
}

impl IsSunObject for Math {
    fn get_obj(&self) -> &SunObject {
        self.class.get_obj()
    }

    fn get_mut_obj(&mut self) -> &mut SunObject {
        self.class.get_mut_obj()
    }
}

impl ImportAble for Math {
    fn export_lib() -> ExportLib {
        let mut meta = HashMap::new();
        let value = HashMap::new();
        meta.insert("Math".to_string(), Math::new().get_obj().clone());
        ExportLib::new(meta, value)
    }
}

#[no_mangle]
pub extern "C" fn export_libc() -> ExportLibC {
    tran::to_c(Math::export_lib())
}

use crate::math::meta::{
    base::{abs, acos, acosh, asin, asinh, atan, atanh, cos, cosh, ln, log, sin, sinh, tan, tanh},
    complex::complex,
};
use sun_core::{
    add_methods,
    meta::{OwnSunMeta, SunBase, SunMeta},
    SunClass,
};

pub struct MathMeta {
    meta: SunMeta,
}

impl MathMeta {
    pub fn new() -> Self {
        let mut class = SunClass::new("Math", SunBase::Object);
        let meta = class.get_meta_mut();
        add_methods!(
            meta,
            ("abs", abs),
            ("cpx", complex),
            ("sin", sin),
            ("cos", cos),
            ("tan", tan),
            ("asin", asin),
            ("acos", acos),
            ("atan", atan),
            ("sinh", sinh),
            ("cosh", cosh),
            ("tanh", tanh),
            ("asinh", asinh),
            ("acosh", acosh),
            ("atanh", atanh),
            ("ln", ln),
            ("log", log)
        );
        MathMeta { meta: meta.clone() }
    }
}

impl OwnSunMeta for MathMeta {
    fn get_meta(&self) -> &sun_core::meta::SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

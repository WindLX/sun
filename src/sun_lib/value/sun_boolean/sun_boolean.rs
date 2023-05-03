use super::super::{
    sun_function::Function,
    sun_meta::{
        op::{AndAble, NotAble, OrAble, XorAble},
        OwnSunMeta,
    },
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::{add_methods, double_op_b, single_op_b};

#[derive(Clone, Debug)]
pub struct SunBoolean {
    obj: SunObject,
}

impl IsSunObject for SunBoolean {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
    }
}

impl SunBoolean {
    pub fn new() -> SunBoolean {
        let mut obj = SunObject::new("bool");
        add_methods!(
            obj,
            SunBoolean,
            ("and", and),
            ("or", or),
            ("xor", xor),
            ("not", not)
        );
        SunBoolean { obj }
    }
}

impl AndAble for SunBoolean {
    fn and() -> Function {
        double_op_b!(&&)
    }
}

impl OrAble for SunBoolean {
    fn or() -> Function {
        double_op_b!(||)
    }
}

impl NotAble for SunBoolean {
    fn not() -> Function {
        single_op_b!(!)
    }
}

impl XorAble for SunBoolean {
    fn xor() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Boolean(b1), SunValue::Boolean(b2)) => {
                    let value = !(b1 == b2);
                    *self_value = SunValue::from(value);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f
    }
}

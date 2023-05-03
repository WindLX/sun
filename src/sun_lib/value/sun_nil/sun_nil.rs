use super::super::{
    sun_function::Function,
    sun_meta::{
        op::{
            AddAble, AndAble, ConjAble, DivAble, FacAble, MulAble, NegAble, NotAble, OrAble,
            PowAble, RemAble, SubAble, XorAble,
        },
        OwnSunMeta,
    },
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::add_methods;

#[derive(Clone, Debug)]
pub struct SunNil {
    obj: SunObject,
}

impl IsSunObject for SunNil {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
    }
}

impl SunNil {
    pub fn new() -> SunNil {
        let mut obj = SunObject::new("nil");
        add_methods!(
            obj,
            SunNil,
            ("add", add),
            ("sub", sub),
            ("mul", mul),
            ("div", div),
            ("rem", rem),
            ("pow", pow),
            ("fac", fac),
            ("neg", neg),
            ("and", and),
            ("or", or),
            ("xor", xor),
            ("not", not)
        );
        SunNil { obj }
    }
}

macro_rules! nil_op {
    () => {{
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            if let SunValue::Nil = arg_0 {
                vec![SunPointer::new(SunValue::Nil)]
            } else {
                vec![]
            }
        };
        f
    }};
}

impl AddAble for SunNil {
    fn add() -> Function {
        nil_op!()
    }
}

impl SubAble for SunNil {
    fn sub() -> Function {
        nil_op!()
    }
}

impl MulAble for SunNil {
    fn mul() -> Function {
        nil_op!()
    }
}

impl DivAble for SunNil {
    fn div() -> Function {
        nil_op!()
    }
}

impl RemAble for SunNil {
    fn rem() -> Function {
        nil_op!()
    }
}

impl NegAble for SunNil {
    fn neg() -> Function {
        nil_op!()
    }
}

impl PowAble for SunNil {
    fn pow() -> Function {
        nil_op!()
    }
}

impl FacAble for SunNil {
    fn fac() -> Function {
        nil_op!()
    }
}

impl ConjAble for SunNil {
    fn conj() -> Function {
        nil_op!()
    }
}

impl AndAble for SunNil {
    fn and() -> Function {
        nil_op!()
    }
}

impl OrAble for SunNil {
    fn or() -> Function {
        nil_op!()
    }
}

impl XorAble for SunNil {
    fn xor() -> Function {
        nil_op!()
    }
}

impl NotAble for SunNil {
    fn not() -> Function {
        nil_op!()
    }
}

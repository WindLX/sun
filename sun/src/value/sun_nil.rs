use sun_core::{
    add_methods,
    container::{Function, RustFunction, SunValue},
    meta::{
        meta_methods::op::{
            AddAble, AndAble, CompareAble, ConjAble, DivAble, FacAble, MulAble, NegAble, NotAble,
            OrAble, PowAble, RemAble, SubAble, XorAble,
        },
        OwnSunMeta,
    },
    utils::{IsSunObject, SunObject, SunPointer},
};

/// Nil 的元数据
#[derive(Clone, Debug)]
pub struct SunNil {
    obj: SunObject,
}

impl IsSunObject for SunNil {
    fn get_obj(&self) -> &SunObject {
        &self.obj
    }

    fn get_mut_obj(&mut self) -> &mut SunObject {
        &mut self.obj
    }
}

impl SunNil {
    /// 创建新的 Nil 元数据
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

/// 批量处理 Nil 的运算符
macro_rules! nil_op {
    () => {{
        let f = |_: Vec<SunPointer>| vec![SunPointer::new(SunValue::Nil)];
        Function::from(f as RustFunction)
    }};
}

/// 批量处理 Nil 的比较运算符
macro_rules! nil_compare {
    () => {{
        let f = |_: Vec<SunPointer>| vec![SunPointer::new(SunValue::from(false))];
        Function::from(f as RustFunction)
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

impl CompareAble for SunNil {
    fn eq() -> Function {
        nil_compare!()
    }

    fn ge() -> Function {
        nil_compare!()
    }

    fn greater() -> Function {
        nil_compare!()
    }

    fn le() -> Function {
        nil_compare!()
    }

    fn less() -> Function {
        nil_compare!()
    }

    fn noteq() -> Function {
        nil_compare!()
    }
}

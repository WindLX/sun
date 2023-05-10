use crate::{compare_op_b, double_op_b, single_op_b};
use sun_core::{
    add_meta_methods,
    container::{Function, RustFunction, SunValue},
    meta::{
        meta_methods::op::{AndAble, CompareAble, NotAble, OrAble, XorAble},
        OwnSunMeta, SunBase, SunMeta,
    },
    utils::SunPointer,
};

/// Bool 元数据
#[derive(Clone, Debug)]
pub struct SunBoolean {
    meta: SunMeta,
}

impl SunBoolean {
    /// 创建新的 Bool 元数据
    pub fn new() -> SunBoolean {
        let mut meta = SunMeta::new("bool", SunBase::Object);
        add_meta_methods!(
            meta,
            SunBoolean,
            ("and", and),
            ("or", or),
            ("xor", xor),
            ("not", not),
            ("eq", eq),
            ("noteq", noteq),
            ("le", le),
            ("ge", ge),
            ("greater", greater),
            ("less", less)
        );
        SunBoolean { meta }
    }
}

impl OwnSunMeta for SunBoolean {
    fn get_meta(&self) -> &SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
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
        Function::from(f as RustFunction)
    }
}

impl CompareAble for SunBoolean {
    fn eq() -> Function {
        compare_op_b!(==)
    }

    fn ge() -> Function {
        compare_op_b!(>=)
    }

    fn greater() -> Function {
        compare_op_b!(>)
    }

    fn le() -> Function {
        compare_op_b!(<=)
    }

    fn less() -> Function {
        compare_op_b!(<)
    }

    fn noteq() -> Function {
        compare_op_b!(!=)
    }
}

/// 批量处理 Bool 的二元操作符
#[macro_export]
macro_rules! double_op_b {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Boolean(b1), SunValue::Boolean(b2)) => {
                    let value = b1 $op b2;
                    *self_value = SunValue::from(value);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        Function::from(f as RustFunction)
    }};
}

/// 批量处理 Bool 的一元操作符
#[macro_export]
macro_rules! single_op_b {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let mut self_value = args[0].borrow_mut();
            let res = match arg {
                SunValue::Boolean(b) => {
                    *self_value = SunValue::from($op b);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        Function::from(f as RustFunction)
    }};
}

/// 批量处理 Bool 的比较操作符
#[macro_export]
macro_rules! compare_op_b {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let res = match (arg_0, arg_1) {
                (SunValue::Boolean(b1), SunValue::Boolean(b2)) => {
                    let value = b1 $op b2;
                    vec![SunPointer::new(SunValue::from(value))]
                }
                _ => Vec::new(),
            };
            res
        };
    Function::from(f as RustFunction)
    }};
}

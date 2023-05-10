use crate::{compare_op, double_op, single_op};
use colorized::*;
use sun_core::{
    add_meta_methods,
    container::{Function, RustFunction, SunValue},
    meta::{
        meta_methods::op::{
            AddAble, CompareAble, DivAble, FacAble, MulAble, NegAble, PowAble, RemAble, SubAble,
        },
        OwnSunMeta, SunBase, SunMeta,
    },
    utils::{
        log::{error_output, warn_output},
        SunError, SunPointer,
    },
};

/// Number 元数据
#[derive(Clone, Debug)]
pub struct SunNumber {
    meta: SunMeta,
}

impl OwnSunMeta for SunNumber {
    fn get_meta(&self) -> &SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

impl SunNumber {
    /// 创建新的 Number 元数据
    pub fn new() -> SunNumber {
        let mut meta = SunMeta::new("number", SunBase::Object);
        add_meta_methods!(
            meta,
            SunNumber,
            ("add", add),
            ("sub", sub),
            ("mul", mul),
            ("div", div),
            ("rem", rem),
            ("pow", pow),
            ("fac", fac),
            ("neg", neg),
            ("eq", eq),
            ("noteq", noteq),
            ("le", le),
            ("ge", ge),
            ("greater", greater),
            ("less", less)
        );
        SunNumber { meta }
    }
}

impl AddAble for SunNumber {
    fn add() -> Function {
        double_op!(+)
    }
}

impl SubAble for SunNumber {
    fn sub() -> Function {
        double_op!(-)
    }
}

impl MulAble for SunNumber {
    fn mul() -> Function {
        double_op!(*)
    }
}

impl DivAble for SunNumber {
    fn div() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1 / n2;
                    if value == f64::INFINITY {
                        warn_output(
                            format!("`{n2}` is zero so when div them will got `infinity`")
                                .color(Colors::YellowFg),
                        );
                    }
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

impl RemAble for SunNumber {
    fn rem() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    if n1.fract() != 0.0 || n2.fract() != 0.0 {
                        warn_output(format!("parameter is not an integer so when take it's remainder there may be problems").color(Colors::YellowFg));
                    }
                    let value = n1 % n2;
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

impl NegAble for SunNumber {
    fn neg() -> Function {
        single_op!(-)
    }
}

impl PowAble for SunNumber {
    fn pow() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1.powf(n2);
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

impl FacAble for SunNumber {
    fn fac() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let mut self_value = args[0].borrow_mut();
            let res = match arg {
                SunValue::Number(n) => {
                    if n.fract() != 0.0 {
                        warn_output(format!("`{n}` is not an integer so when take it's factorial will ignore the fractional part").color(Colors::YellowFg));
                    }
                    let n = n as i64;
                    if n >= 21 {
                        let e = SunError::ParaError(format!("too big number `{n}` for factorial"));
                        error_output(e);
                    }
                    let mut res = 1;
                    if n != 0 {
                        for i in 1..=n {
                            res *= i;
                        }
                    }
                    *self_value = SunValue::from(res as f64);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        Function::from(f as RustFunction)
    }
}

impl CompareAble for SunNumber {
    fn eq() -> Function {
        compare_op!(==)
    }

    fn ge() -> Function {
        compare_op!(>=)
    }

    fn greater() -> Function {
        compare_op!(>)
    }

    fn le() -> Function {
        compare_op!(<=)
    }

    fn less() -> Function {
        compare_op!(<)
    }

    fn noteq() -> Function {
        compare_op!(!=)
    }
}

/// 批量处理 Number 的二元操作符
#[macro_export]
macro_rules! double_op {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1 $op n2;
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

/// 批量处理 Number 的一元操作符
#[macro_export]
macro_rules! single_op {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let mut self_value = args[0].borrow_mut();
            let res = match arg {
                SunValue::Number(n) => {
                    *self_value = SunValue::from($op n);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        Function::from(f as RustFunction)
    }};
}

/// 批量处理 Number 的比较操作符
#[macro_export]
macro_rules! compare_op {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1 $op n2;
                    vec![SunPointer::new(SunValue::from(value))]
                }
                _ => Vec::new(),
            };
            res
        };
        Function::from(f as RustFunction)
    }};
}

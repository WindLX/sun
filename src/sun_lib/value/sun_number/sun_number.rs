use super::super::{
    sun_function::Function,
    sun_meta::{
        op::{AddAble, DivAble, FacAble, MulAble, NegAble, PowAble, RemAble, SubAble},
        OwnSunMeta,
    },
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::utils::{
    err::SunError,
    log::{error_output, warn_output},
};
use crate::{add_methods, double_op, single_op};
use colorized::*;

#[derive(Clone, Debug)]
pub struct SunNumber {
    obj: SunObject,
}

impl IsSunObject for SunNumber {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
    }
}

impl SunNumber {
    pub fn new() -> SunNumber {
        let mut obj = SunObject::new("number");
        add_methods!(
            obj,
            SunNumber,
            ("add", add),
            ("sub", sub),
            ("mul", mul),
            ("div", div),
            ("rem", rem),
            ("pow", pow),
            ("fac", fac),
            ("neg", neg)
        );
        SunNumber { obj }
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
        f
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
        f
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
        f
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
        f
    }
}

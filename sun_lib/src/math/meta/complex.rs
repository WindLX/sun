use crate::math::container::Complex;
use sun_core::{
    container::{Function, IsSunClass, RustFunction, SunValue},
    utils::{log::error_output, SunError, SunPointer},
};

/// 新建复数
pub fn complex() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let i = args[0].get();
        let r = args[1].get();
        let res = match (i, r) {
            (SunValue::Number(i), SunValue::Number(r)) => {
                let cpx = SunValue::Class(Complex::new(i, r).get_class().clone());
                vec![SunPointer::new(cpx)]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

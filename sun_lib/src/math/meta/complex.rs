use crate::{double_op_cpx, math::container::Complex};
use sun_core::{
    add_meta_methods, add_methods,
    container::{Function, IsSunClass, RustFunction, SunValue, Table},
    meta::{
        meta_methods::{
            converter::Converter,
            op::{AddAble, ConjAble, DivAble, EqualAble, MulAble, NegAble, PowAble, SubAble},
        },
        OwnSunMeta, SunBase, SunMeta,
    },
    utils::{log::error_output, SunError, SunPointer},
    SunClass,
};

pub struct ComplexMeta {
    meta: SunMeta,
}

impl ComplexMeta {
    pub fn new() -> Self {
        let mut class = SunClass::new("Complex", SunBase::Other("Math".to_string()));
        let meta = class.get_meta_mut();
        add_methods!(meta, ("mag", magnitude), ("euler", euler));
        add_meta_methods!(
            meta,
            ComplexMeta,
            ("add", add),
            ("sub", sub),
            ("mul", mul),
            ("div", div),
            ("pow", pow),
            ("conj", conj),
            ("neg", neg),
            ("eq", eq),
            ("noteq", noteq),
            ("to", to)
        );
        meta.set_method("from", <ComplexMeta as Converter>::from());
        ComplexMeta { meta: meta.clone() }
    }
}

impl OwnSunMeta for ComplexMeta {
    fn get_meta(&self) -> &sun_core::meta::SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

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

/// 取模
pub fn magnitude() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let cpx = Complex::from(args[0].get());
        let mut self_value = args[0].borrow_mut();
        *self_value = SunValue::from(cpx.get_mag());
        let res = vec![args[0].clone()];
        res
    };
    Function::from(f as RustFunction)
}

/// 转换成欧拉模式
pub fn euler() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let cpx = Complex::from(args[0].get());
        let mut self_value = args[0].borrow_mut();
        let (rho, theta) = cpx.to_euler();
        let mut res = Table::new();
        res.append_kv("rho".to_string(), SunValue::from(rho));
        res.append_kv("theta".to_string(), SunValue::from(theta));
        *self_value = SunValue::from(res);
        vec![args[0].clone()]
    };
    Function::from(f as RustFunction)
}

impl AddAble for ComplexMeta {
    fn add() -> Function {
        double_op_cpx!(+)
    }
}

impl SubAble for ComplexMeta {
    fn sub() -> Function {
        double_op_cpx!(-)
    }
}

impl MulAble for ComplexMeta {
    fn mul() -> Function {
        double_op_cpx!(*)
    }
}

impl DivAble for ComplexMeta {
    fn div() -> Function {
        double_op_cpx!(/)
    }
}

impl ConjAble for ComplexMeta {
    fn conj() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() < 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx = Complex::from(args[0].get());
            let mut self_value = args[0].borrow_mut();
            *self_value = SunValue::Class(cpx.get_conj().get_class().clone());
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }
}

impl NegAble for ComplexMeta {
    fn neg() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() < 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx = Complex::from(args[0].get());
            let mut self_value = args[0].borrow_mut();
            *self_value = SunValue::Class((-cpx).get_class().clone());
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }
}

impl PowAble for ComplexMeta {
    fn pow() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() <= 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx_0 = Complex::from(args[0].get());
            let cpx_1 = Complex::from(args[1].get());
            let mut self_value = args[0].borrow_mut();
            *self_value = SunValue::Class(cpx_0.get_power(cpx_1).get_class().clone());
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }
}

impl EqualAble for ComplexMeta {
    fn eq() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() <= 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx_0 = Complex::from(args[0].get());
            let cpx_1 = Complex::from(args[1].get());
            let value = cpx_0 == cpx_1;
            let res = vec![SunPointer::new(SunValue::from(value))];
            res
        };
        Function::from(f as RustFunction)
    }

    fn noteq() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() <= 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx_0 = Complex::from(args[0].get());
            let cpx_1 = Complex::from(args[1].get());
            let value = cpx_0 != cpx_1;
            let res = vec![SunPointer::new(SunValue::from(value))];
            res
        };
        Function::from(f as RustFunction)
    }
}

impl Converter for ComplexMeta {
    fn from() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() < 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx = Complex::from(args[0].get());
            let mut self_value = args[0].borrow_mut();
            *self_value = SunValue::Class(cpx.get_class().clone());
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }

    fn to() -> Function {
        let f = |args: Vec<SunPointer>| {
            if args.len() < 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let tb = SunValue::from(Complex::from(args[0].get()));
            let mut self_value = args[0].borrow_mut();
            *self_value = tb;
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }
}

#[macro_export]
macro_rules! double_op_cpx {
    ($op:tt) => {{
        let f = |args: Vec<SunPointer>| {
            if args.len() <= 1 {
                {
                    let e = SunError::ParaError(format!("the number of parameters is too few"));
                    error_output(e);
                }
            }
            let cpx_0 = Complex::from(args[0].get());
            let cpx_1 = Complex::from(args[1].get());
            let mut self_value = args[0].borrow_mut();
            *self_value = SunValue::Class((cpx_0 $op cpx_1).get_class().clone());
            let res = vec![args[0].clone()];
            res
        };
        Function::from(f as RustFunction)
    }};
}

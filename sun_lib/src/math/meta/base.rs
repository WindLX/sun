use crate::math::container::Complex;
use sun_core::{
    container::{Function, IsSunClass, RustFunction, SunValue},
    utils::{log::error_output, SunError, SunPointer},
};

/// 计算绝对值
pub fn abs() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.abs());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算正弦
pub fn sin() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.sin());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算余弦
pub fn cos() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.cos());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算正切
pub fn tan() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.tan());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反正弦
pub fn asin() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.asin());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反余弦
pub fn acos() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.acos());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反正切
pub fn atan() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.atan());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算正弦h
pub fn sinh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.sinh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算余弦h
pub fn cosh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.cosh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算正切h
pub fn tanh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.tanh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反正弦h
pub fn asinh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.asinh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反余弦h
pub fn acosh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.acosh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算反正切h
pub fn atanh() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.atanh());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

/// 计算自然对数
pub fn ln() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = Complex::from(arg).get_ln();
        *self_value = SunValue::Class(res.get_class().clone());
        vec![args[0].clone()]
    };
    Function::from(f as RustFunction)
}

/// 计算任意值为底的对数
pub fn log() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 2 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let arg = args[0].get();
        let base = args[1].get();
        let mut self_value = args[0].borrow_mut();
        let res = Complex::from(arg).get_log(Complex::from(base));
        *self_value = SunValue::Class(res.get_class().clone());
        vec![args[0].clone()]
    };
    Function::from(f as RustFunction)
}

use crate::sun_lib::utils::complex_index;
use crate::sun_lib::{sun_value::SunValue, Includable, Preludable, SunMod};
use crate::utils::err::SunError;
use crate::vm::machine::VirtualMachine;
use crate::{consts_none, docs_none, funcs, funcs_none};
use std::process;

pub struct Op;

macro_rules! double_op {
    ($func_name:ident, $op:tt) => {
        fn $func_name(state: &mut VirtualMachine) -> u8 {
            match (state.pop(), state.pop()) {
                (Some(n1 @ _), Some(n2 @ _)) => {
                    state.push(n1 $op n2);
                }
                _ => {
                    eprintln!("{}", SunError::TypeError(concat!("wrong type of parameter for `", stringify!($op), "`").to_string()));
                    process::exit(0)
                },
            }
            2
        }
    };
}

macro_rules! double_bool_op {
    ($func_name:ident, $op:tt) => {
        fn $func_name(state: &mut VirtualMachine) -> u8 {
            match (state.pop(), state.pop()) {
                (Some(SunValue::Boolean(b1)), Some(SunValue::Boolean(b2))) => {
                    state.push(SunValue::from(b2 $op b1));
                }
                _ => {
                    eprintln!("{}", SunError::TypeError(concat!("wrong type of parameter for `", stringify!($func_name), "`").to_string()));
                    process::exit(0)
                },
            }
            2
        }

    };
}

double_op!(add, +);
double_op!(sub, -);
double_op!(mul, *);
double_op!(div, /);
double_op!(rem, %);

double_bool_op!(and, &&);
double_bool_op!(or, ||);

fn neg(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(-n)),
        Some(SunValue::Complex(r, i)) => state.push(SunValue::from((-r, -i))),
        _ => state.push(SunValue::Nil),
    }
    1
}

fn fac(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => {
            let n = n as i64;
            let mut res = 1;
            if n != 0 {
                for i in 1..=n {
                    res *= i;
                }
            }
            state.push(SunValue::from(res as f64))
        }
        _ => state.push(SunValue::Nil),
    }
    1
}

fn pow(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Number(n1)), Some(SunValue::Number(n2))) => {
            state.push(SunValue::from(n1.powf(n2)));
        }
        (Some(SunValue::Number(n1)), Some(SunValue::Complex(r2, i2))) => {
            state.push(SunValue::from(complex_index((n1, 0.0), (r2, i2))));
        }
        (Some(SunValue::Complex(r1, i1)), Some(SunValue::Number(n2))) => {
            state.push(SunValue::from(complex_index((r1, i1), (n2, 0.0))));
        }
        (Some(SunValue::Complex(r1, i1)), Some(SunValue::Complex(r2, i2))) => {
            state.push(SunValue::from(complex_index((r1, i1), (r2, i2))));
        }
        _ => state.push(SunValue::Nil),
    }
    2
}

fn not(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Boolean(b)) => {
            state.push(SunValue::Boolean(!b));
        }
        _ => state.push(SunValue::Nil),
    }
    1
}

fn conj(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Complex(r, i)) => {
            state.push(SunValue::Complex(r, -i));
        }
        _ => state.push(SunValue::Nil),
    }
    1
}

fn xor(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Boolean(b1)), Some(SunValue::Boolean(b2))) => {
            if b1 == b2 {
                state.push(SunValue::Boolean(false))
            } else {
                state.push(SunValue::Boolean(true))
            }
        }
        _ => state.push(SunValue::Nil),
    }
    2
}

impl Includable for Op {
    fn generate_mod(&self) -> SunMod {
        let docs = docs_none!();
        SunMod::new("op".to_string(), docs, consts_none!(), funcs_none!())
    }
}

impl Preludable for Op {
    fn generate_preclude_mod(&self) -> std::collections::HashMap<String, SunValue> {
        funcs!(add, sub, mul, div, rem, and, or, neg, fac, pow, not, conj, xor)
    }
}

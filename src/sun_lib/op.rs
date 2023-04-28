use crate::include_func;
use crate::sun_lib::include::Include;
use crate::vm::{machine::VirtualMachine, value::SunValue};
use std::collections::HashMap;
use std::f64::INFINITY;

pub struct Op;

macro_rules! double_op {
    ($func_name:ident, $op:tt) => {
        fn $func_name(state: &mut VirtualMachine) -> u8 {
            match (state.pop(), state.pop()) {
                (Some(SunValue::Number(n1)), Some(SunValue::Number(n2))) => {
                    let res = n2 $op n1;
                    if res == INFINITY {
                        state.push(SunValue::Nil);
                    } else {
                        state.push(SunValue::Number(res));
                    }
                }
                _ => panic!(concat!("TODO: ", stringify!($name), " error")),
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
                    state.push(SunValue::Boolean(b2 $op b1));
                }
                _ => panic!(concat!("TODO: ", stringify!($name), " error")),
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
        Some(SunValue::Number(n)) => state.push(SunValue::Number(-n)),
        _ => panic!("TODO: neg error"),
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
            state.push(SunValue::Number(res as f64))
        }
        _ => panic!("TODO: fac error"),
    }
    1
}

fn pow(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Number(n1)), Some(SunValue::Number(n2))) => {
            state.push(SunValue::Number(n1.powf(n2)));
        }
        _ => panic!("TODO: pow error"),
    }
    2
}

fn not(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Boolean(b)) => {
            state.push(SunValue::Boolean(!b));
        }
        _ => panic!("TODO: not error"),
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
        _ => panic!("TODO: xor error"),
    }
    2
}

impl Include for Op {
    fn include(global_map: &mut HashMap<String, SunValue>) {
        include_func!(global_map, add, sub, mul, div, rem, pow, neg, fac, and, or, not, xor);
    }
}

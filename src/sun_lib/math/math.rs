use super::utils::*;
use crate::sun_lib::{sun_value::SunValue, Includable, Preludable, SunMod};
use crate::utils::err::SunError;
use crate::vm::machine::VirtualMachine;
use crate::{consts, consts_none, docs_none, funcs, funcs_none};
use std::f64::consts;
use std::process;

pub struct Math;

const E: SunValue = SunValue::Number(consts::E);
const PI: SunValue = SunValue::Number(consts::PI);
const I: SunValue = SunValue::Complex(0f64, 1f64);

fn abs(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(nn)) => state.push(SunValue::from(nn.abs())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `abs`"))
            );
            process::exit(0)
        }
    }
    1
}

fn sin(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.sin())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `sin`"))
            );
            process::exit(0)
        }
    }
    1
}

fn cos(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.cos())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `cos`"))
            );
            process::exit(0)
        }
    }
    1
}

fn tan(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.tan())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `tan`"))
            );
            process::exit(0)
        }
    }
    1
}

fn asin(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.asin())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `asin`"))
            );
            process::exit(0)
        }
    }
    1
}

fn acos(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.acos())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `acos`"))
            );
            process::exit(0)
        }
    }
    1
}

fn atan(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.atan())),
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `atan`"))
            );
            process::exit(0)
        }
    }
    1
}

fn ln(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(n)) => state.push(SunValue::from(n.ln())),
        Some(c @ SunValue::Complex(_, _)) => match complex_ln(&c) {
            Ok(c) => state.push(SunValue::from(c)),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(0);
            }
        },
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `ln`"))
            );
            process::exit(0)
        }
    }
    1
}

fn log(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Number(base)), Some(SunValue::Number(n))) => {
            state.push(SunValue::from(n.log(base)))
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `log`"))
            );
            process::exit(0)
        }
    }
    2
}

fn cpx(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Number(r)), Some(SunValue::Number(i))) => {
            state.push(SunValue::from((r, i)))
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `cpx`"))
            );
            process::exit(0)
        }
    }
    2
}

fn mag(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Complex(r, i)) => {
            state.push(SunValue::from((r.powi(2) + i.powi(2)).powf(0.5)))
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `mag`"))
            );
            process::exit(0)
        }
    }
    1
}

impl Includable for Math {
    fn generate_mod(&self) -> SunMod {
        let docs = docs_none!();
        SunMod::new("math".to_string(), docs, consts_none!(), funcs_none!())
    }
}

impl Preludable for Math {
    fn generate_preclude_mod(&self) -> std::collections::HashMap<String, SunValue> {
        let mut funcs = funcs!(abs, sin, cos, tan, asin, acos, atan, ln, log, cpx, mag);
        let consts = consts!(I, E, PI);
        funcs.extend(consts);
        funcs
    }
}

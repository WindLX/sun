use crate::sun_lib::include::Include;
use crate::vm::{machine::VirtualMachine, value::SunValue};
use std::collections::HashMap;
use std::f64::consts;

pub struct Math;

const E: SunValue = SunValue::Number(consts::E);
const PI: SunValue = SunValue::Number(consts::PI);
const I: SunValue = SunValue::Complex(0f64, 1f64);

fn abs(state: &mut VirtualMachine) {
    match state.pop() {
        Some(SunValue::Number(nn)) => state.push(SunValue::Number(nn.abs())),
        _ => panic!("TODO: abs error"),
    }
}

fn add(state: &mut VirtualMachine) {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Number(n1)), Some(SunValue::Number(n2))) => {
            state.push(SunValue::Number(n1 + n2));
        }
        _ => panic!("TODO: add error"),
    }
}

impl Include for Math {
    fn include(global_map: &mut HashMap<String, SunValue>) {
        global_map.insert("i".to_string(), I);
        global_map.insert("pi".to_string(), PI);
        global_map.insert("e".to_string(), E);
        global_map.insert("abs".to_string(), SunValue::Function(abs));
        global_map.insert("add".to_string(), SunValue::Function(add));
    }
}

use crate::sun_lib::include::Include;
use crate::vm::{machine::VirtualMachine, value::SunValue};
use crate::{include_const, include_func};
use std::collections::HashMap;
use std::f64::consts;

pub struct Math;

const E: SunValue = SunValue::Number(consts::E);
const PI: SunValue = SunValue::Number(consts::PI);
const I: SunValue = SunValue::Complex(0f64, 1f64);

fn abs(state: &mut VirtualMachine) -> u8 {
    match state.pop() {
        Some(SunValue::Number(nn)) => state.push(SunValue::Number(nn.abs())),
        _ => panic!("TODO: abs error"),
    }
    1
}

impl Include for Math {
    fn include(global_map: &mut HashMap<String, SunValue>) {
        include_func!(global_map, abs);
        include_const!(global_map, PI, E, I);
    }
}

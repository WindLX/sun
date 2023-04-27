use crate::sun_lib::include::Include;
use crate::vm::machine::VirtualMachine;
use crate::vm::value::SunValue;
use std::collections::HashMap;

pub struct IO;

fn print(state: &mut VirtualMachine) {
    let value = state.pop();
    println!("{}", value.unwrap_or(SunValue::Nil));
}

impl Include for IO {
    fn include(global_map: &mut HashMap<String, SunValue>) {
        global_map.insert("print".to_string(), SunValue::Function(print));
    }
}

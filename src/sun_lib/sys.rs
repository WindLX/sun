use crate::sun_lib::include::Include;
use crate::vm::machine::VirtualMachine;
use crate::vm::value::SunValue;
use std::collections::HashMap;
use std::process;

pub struct Sys;

fn exit(_: &mut VirtualMachine) -> u8 {
    process::exit(0);
}

impl Include for Sys {
    fn include(global_map: &mut HashMap<String, SunValue>) {
        global_map.insert("exit".to_string(), SunValue::Function(exit));
    }
}

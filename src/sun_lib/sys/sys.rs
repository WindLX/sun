use crate::sun_lib::{sun_type::SunType, sun_value::SunValue, Includable, Preludable, SunMod};
use crate::utils::err::SunError;
use crate::vm::machine::VirtualMachine;
use crate::{consts_none, docs_none, funcs, funcs_none};
use std::process;

pub struct Sys;

fn exit(_: &mut VirtualMachine) -> u8 {
    process::exit(0);
}

fn drop(state: &mut VirtualMachine) -> u8 {
    let num = state.len();
    if num == 0 {
        return 0;
    }
    for _ in 0..num {
        match state.pop() {
            Some(SunValue::String(key)) => state.remove(key.as_str()),
            Some(other) => {
                eprintln!(
                    "{}",
                    SunError::TypeError(format!(
                        "invalid variable name: `{}`",
                        SunType::from(other)
                    ))
                );
                process::exit(0)
            }
            None => return 0,
        }
    }
    num as u8
}

impl Includable for Sys {
    fn generate_mod(&self) -> SunMod {
        let docs = docs_none!();
        SunMod::new("sys".to_string(), docs, consts_none!(), funcs_none!())
    }
}

impl Preludable for Sys {
    fn generate_preclude_mod(&self) -> std::collections::HashMap<String, SunValue> {
        funcs!(exit, drop)
    }
}

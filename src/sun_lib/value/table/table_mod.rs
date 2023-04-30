use super::Table;
use crate::sun_lib::pointer::SunPointer;
use crate::sun_lib::{sun_value::SunValue, Includable, Preludable, SunMod};
use crate::utils::err::SunError;
use crate::vm::machine::VirtualMachine;
use crate::{consts_none, docs_none, funcs, funcs_none};
use std::process;

pub struct TableMod;

fn table(state: &mut VirtualMachine) -> u8 {
    state.push(SunValue::from(SunPointer::new(SunValue::Table(
        Table::new(),
    ))));
    0
}

fn push(state: &mut VirtualMachine) -> u8 {
    let n = state.len();
    match state.pop() {
        Some(SunValue::Pointer(p)) => {
            let num = state.len();
            for _ in 0..num {
                match state.pop() {
                    Some(v) => match p.push_by_index(v) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("{e}");
                            process::exit(0);
                        }
                    },
                    None => {
                        eprintln!(
                            "{}",
                            SunError::TypeError(format!("wrong type of parameter for `push`"))
                        );
                        process::exit(0);
                    }
                }
            }
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `push`"))
            );
            process::exit(0);
        }
    }
    n as u8
}

fn insert(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop(), state.pop()) {
        (Some(SunValue::Pointer(p)), Some(SunValue::String(k)), Some(v)) => {
            match p.insert_by_kv(k.to_string(), v) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(0);
                }
            }
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `insert`"))
            );
            process::exit(0);
        }
    }
    3
}

fn remove(state: &mut VirtualMachine) -> u8 {
    match (state.pop(), state.pop()) {
        (Some(SunValue::Pointer(p)), Some(SunValue::String(k))) => {
            match p.remove_by_key(k.to_string()) {
                Ok(v) => state.push(v),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(0);
                }
            }
        }
        (Some(SunValue::Pointer(p)), Some(SunValue::Number(i))) => {
            match p.remove_by_index(i as usize) {
                Ok(v) => state.push(v),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(0);
                }
            }
        }
        _ => {
            eprintln!(
                "{}",
                SunError::TypeError(format!("wrong type of parameter for `remove`"))
            );
            process::exit(0);
        }
    }
    2
}

impl Includable for TableMod {
    fn generate_mod(&self) -> SunMod {
        let docs = docs_none!();
        SunMod::new("table_mod".to_string(), docs, consts_none!(), funcs_none!())
    }
}

impl Preludable for TableMod {
    fn generate_preclude_mod(&self) -> std::collections::HashMap<String, SunValue> {
        let funcs = funcs!(table, push, insert, remove);
        funcs
    }
}

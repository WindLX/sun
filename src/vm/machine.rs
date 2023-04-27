use crate::sun_lib::{include::Include, io::IO, math::Math};
use crate::utils::err::SunError;
use crate::vm::value::{SunFunc, SunValue};
use crate::{parser::parser::ParseProto, vm::command::Command};
use std::collections::HashMap;
use std::io::Read;
use std::process;

pub struct VirtualMachine {
    stack: Vec<SunValue>,
    call_stack: Vec<SunFunc>,
    value_map: HashMap<String, SunValue>,
    is_debug: bool,
}

impl VirtualMachine {
    pub fn new(is_debug: bool) -> Self {
        let mut value_map = HashMap::new();
        IO::include(&mut value_map);
        Math::include(&mut value_map);
        VirtualMachine {
            stack: Vec::new(),
            call_stack: Vec::new(),
            value_map,
            is_debug,
        }
    }

    pub fn run<T: Read>(&mut self, proto: &ParseProto<T>) {
        for (index, command) in proto.commands.iter().enumerate() {
            if self.is_debug == true {
                println!("index: {index}, command: {command:?}");
            }
            match &command {
                Command::LoadValue(name) => {
                    let value = self
                        .value_map
                        .get(name.as_str())
                        .unwrap_or(&SunValue::Nil)
                        .clone();
                    self.stack.push(value);
                }
                Command::SetGlobalValue(name) => {
                    self.value_map
                        .insert(name.clone(), self.stack.pop().unwrap_or(SunValue::Nil));
                }
                Command::CopyValue(source, target) => {
                    let value = self
                        .value_map
                        .get(source.as_str())
                        .unwrap_or(&SunValue::Nil)
                        .clone();
                    self.value_map.insert(target.clone(), value);
                }
                Command::AddValue(value) => {
                    self.stack.push(value.clone());
                }
                Command::SetValue(name, value) => {
                    self.value_map.insert(name.clone(), value.clone());
                }
                Command::LoadFunc(name) => {
                    let name: &str = name.as_str();
                    let value = self.value_map.get(name).clone();
                    match value {
                        Some(SunValue::Function(f)) => self.call_stack.push(*f),
                        _ => {
                            eprintln!(
                                "{}",
                                SunError::CallError(
                                    format!("failed to find function {name}"),
                                    index as u64
                                )
                            );
                            process::exit(-1);
                        }
                    }
                }
                Command::CreateTensor => {
                    let new_tensor: Vec<SunValue> = Vec::new();
                    self.stack.push(SunValue::Tensor(new_tensor));
                }
                Command::SetTensor(target_index) => match self.stack.pop() {
                    Some(SunValue::Tensor(mut t)) => {
                        if *target_index as usize > t.len() - 1 {
                            t.push(self.stack.pop().unwrap_or(SunValue::Nil));
                        } else {
                            t[*target_index as usize] = self.stack.pop().unwrap_or(SunValue::Nil);
                        }
                    }
                    _ => panic!("TODO: assign tensor error"),
                },
                Command::Call => match self.call_stack.pop() {
                    Some(f) => {
                        f(self);
                    }
                    None => {
                        eprintln!(
                            "{}",
                            SunError::CallError(
                                "failed to find function".to_string(),
                                index as u64
                            )
                        );
                        process::exit(-1);
                    }
                },
            }
        }
    }

    pub fn pop(&mut self) -> Option<SunValue> {
        self.stack.pop()
    }

    pub fn push(&mut self, value: SunValue) {
        self.stack.push(value);
    }
}

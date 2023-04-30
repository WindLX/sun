use crate::sun_lib::pointer::SunPointer;
use crate::sun_lib::{
    sun_type::SunType, sun_value::SunValue, table::TableMod, Includable, Math, Op, Preludable, Sys,
    IO,
};
use crate::utils::err::SunError;
use crate::{parser::parser::ParseProto, vm::command::Command};
use std::collections::HashMap;
use std::io::Read;
use std::process;

pub struct VirtualMachine {
    stack: Vec<SunValue>,
    value_map: HashMap<String, SunValue>,
    is_debug: bool,
}

impl VirtualMachine {
    pub fn new(is_debug: bool) -> Self {
        let mut vm = VirtualMachine {
            stack: Vec::new(),
            value_map: HashMap::new(),
            is_debug,
        };
        vm.preclude();
        vm
    }

    pub fn run<T: Read>(&mut self, proto: &ParseProto<T>) {
        for (index, command) in proto.commands.iter().enumerate() {
            match &command {
                Command::LoadValue(name) => {
                    let value = self.value_map.get(name.as_str()).unwrap_or(&SunValue::Nil);
                    match value {
                        SunValue::Function(f) => {
                            let value = f.clone();
                            self.stack.push(SunValue::from(value));
                        }
                        SunValue::Pointer(p) => self.stack.push(SunValue::from(p.clone())),
                        SunValue::Nil => self.stack.push(SunValue::from(name.as_str())),
                        _ => {
                            let value = value.clone();
                            self.stack.push(value);
                        }
                    };
                }
                Command::SetGlobalValue(n) => {
                    if n == "_" {
                        self.stack.pop();
                    } else {
                        self.value_map
                            .insert(n.to_string(), self.stack.pop().unwrap_or(SunValue::Nil));
                    }
                }
                Command::AddValue(value) => match value {
                    SunValue::Table(t) => {
                        let value = SunPointer::new(SunValue::from(t.clone()));
                        self.stack.push(SunValue::from(value));
                    }
                    _ => self.stack.push(value.clone()),
                },
                Command::LoadTableValueByKey => match self.stack.pop() {
                    Some(SunValue::Table(t)) => match self.stack.pop() {
                        Some(SunValue::String(k)) => match t.get_by_key(k.as_str()) {
                            Some(p) => {
                                self.stack.push(SunValue::from(p));
                            }
                            None => {
                                eprintln!(
                                    "{}",
                                    SunError::KeyError(format!(
                                        "failed to find value by key `{k}`"
                                    ),)
                                );
                                process::exit(0);
                            }
                        },
                        other => {
                            eprintln!(
                                "{}",
                                SunError::KeyError(format!(
                                    "invalid key `{}`",
                                    SunType::from(other.unwrap_or(SunValue::Nil))
                                ))
                            );
                            process::exit(0);
                        }
                    },
                    Some(SunValue::Pointer(p)) => match p.get_type() {
                        SunType::Table => match self.stack.pop() {
                            Some(SunValue::String(k)) => match p.get_by_key(k.as_str()) {
                                Some(pp) => self.stack.push(SunValue::from(pp)),
                                None => self.stack.push(SunValue::Nil),
                            },
                            other => {
                                eprintln!(
                                    "{}",
                                    SunError::KeyError(format!(
                                        "invalid key `{}`",
                                        SunType::from(other.unwrap_or(SunValue::Nil))
                                    ),)
                                );
                                process::exit(0);
                            }
                        },
                        other => {
                            eprintln!(
                                "{}",
                                SunError::TypeError(format!("expect `table` but got `{}`", other))
                            );
                            process::exit(0);
                        }
                    },
                    other => {
                        eprintln!(
                            "{}",
                            SunError::TypeError(format!(
                                "expect `table` but got `{}`",
                                SunType::from(other.unwrap_or(SunValue::Nil))
                            ))
                        );
                        process::exit(0);
                    }
                },
                Command::LoadTableValueByIndex => match self.stack.pop() {
                    Some(SunValue::Table(t)) => match self.stack.pop() {
                        Some(SunValue::Number(i)) => match t.get_by_index(i as usize) {
                            Some(p) => {
                                self.stack.push(SunValue::from(p));
                            }
                            None => {
                                eprintln!(
                                    "{}",
                                    SunError::IndexError(format!("index `{i}` of range"),)
                                );
                                process::exit(0);
                            }
                        },
                        other => {
                            eprintln!(
                                "{}",
                                SunError::IndexError(format!(
                                    "invalid index `{}`",
                                    SunType::from(other.unwrap_or(SunValue::Nil))
                                ),)
                            );
                            process::exit(0);
                        }
                    },
                    Some(SunValue::Pointer(p)) => match p.get_type() {
                        SunType::Table => match self.stack.pop() {
                            Some(SunValue::Number(i)) => match p.get_by_index(i as usize) {
                                Some(pp) => self.stack.push(SunValue::from(pp)),
                                None => self.stack.push(SunValue::Nil),
                            },
                            other => {
                                eprintln!(
                                    "{}",
                                    SunError::IndexError(format!(
                                        "invalid index `{}`",
                                        SunType::from(other.unwrap_or(SunValue::Nil))
                                    ))
                                );
                                process::exit(0);
                            }
                        },
                        other => {
                            eprintln!(
                                "{}",
                                SunError::TypeError(format!("expect `table` but got `{}`", other))
                            );
                            process::exit(0);
                        }
                    },
                    other => {
                        eprintln!(
                            "{}",
                            SunError::TypeError(format!(
                                "expect `table` but got `{}`",
                                SunType::from(other.unwrap_or(SunValue::Nil))
                            ))
                        );
                        process::exit(0);
                    }
                },
                Command::SetTableValue => match self.stack.pop() {
                    Some(SunValue::Pointer(p)) => match self.stack.pop() {
                        Some(v) => p.set_value(v),
                        None => p.set_value(SunValue::Nil),
                    },
                    other => {
                        eprintln!(
                            "{}",
                            SunError::TypeError(format!(
                                "expect `pointer` but got `{}`",
                                SunType::from(other.unwrap_or(SunValue::Nil))
                            ))
                        );
                        process::exit(0);
                    }
                },
                Command::LoadFunc(name) => {
                    let name: &str = name.as_str();
                    let value = self.value_map.get(name).clone();
                    match value {
                        Some(SunValue::Function(f)) => self.stack.push(SunValue::from(*f)),
                        Some(SunValue::Pointer(p)) => match p.get_func() {
                            Some(f) => {
                                self.stack.push(f);
                            }
                            None => {
                                eprintln!(
                                    "{}",
                                    SunError::CallError(format!("failed to find function {name}"))
                                );
                                process::exit(0);
                            }
                        },
                        _ => {
                            eprintln!(
                                "{}",
                                SunError::CallError(format!("failed to find function {name}"))
                            );
                            process::exit(0);
                        }
                    }
                }
                Command::Call(n) => match self.stack.pop() {
                    Some(SunValue::Function(f)) => {
                        let nn = f(self);
                        if nn != *n {
                            eprintln!(
                                "{}",
                                SunError::ParaError(
                                    format!(
                                        "called function needs `{nn}` parameters, but provide `{n}`"
                                    ),
                                    index as u64
                                )
                            );
                            process::exit(0)
                        }
                    }
                    Some(SunValue::Pointer(p)) => match p.get_func() {
                        Some(f) => {
                            if let SunValue::Function(ff) = f {
                                let nn = ff(self);
                                if nn != *n {
                                    eprintln!(
                                        "{}",
                                        SunError::ParaError(
                                            format!(
                                        "called function needs `{nn}` parameters, but provide `{n}`"
                                    ),
                                            index as u64
                                        )
                                    );
                                    process::exit(0)
                                }
                            }
                        }
                        None => {
                            eprintln!(
                                "{}",
                                SunError::CallError(format!(
                                    "this pointer doesn't point a function"
                                ))
                            );
                            process::exit(0);
                        }
                    },
                    Some(other) => {
                        eprintln!(
                            "{}",
                            SunError::CallError(format!("{} isn't function", SunType::from(other)))
                        );
                        process::exit(0);
                    }
                    None => {
                        eprintln!(
                            "{}",
                            SunError::CallError("failed to find function".to_string())
                        );
                        process::exit(0);
                    }
                },
            }
            if self.is_debug == true {
                println!();
                println!("--- index ---");
                println!("{index}");
                println!("--- command ---");
                println!("command: {command:?}");
                println!("--- stack ---");
                println!("{:?}", self.stack);
                println!("--- value_map ---");
                println!("{:#?}", self.value_map);
                println!();
            }
        }
    }

    fn preclude(&mut self) {
        IO.include(&mut self.value_map);
        Sys.include(&mut self.value_map);
        Math.include(&mut self.value_map);
        Op.include(&mut self.value_map);
        TableMod.include(&mut self.value_map);

        IO.prelude(&mut self.value_map);
        Sys.prelude(&mut self.value_map);
        Math.prelude(&mut self.value_map);
        Op.prelude(&mut self.value_map);
        TableMod.prelude(&mut self.value_map);
    }

    pub fn pop(&mut self) -> Option<SunValue> {
        self.stack.pop()
    }

    pub fn push(&mut self, value: SunValue) {
        self.stack.push(value);
    }

    pub fn len(&mut self) -> usize {
        self.stack.len()
    }

    pub fn remove(&mut self, key: &str) {
        self.value_map.remove(key);
    }
}

use crate::sun_lib::prelude::prelude;
use crate::sun_lib::value::{
    sun_meta::OwnSunMeta,
    sun_object::{SunObject, SunValue},
    sun_pointer::SunPointer,
    sun_table::Table,
};
use crate::utils::{
    err::SunError,
    log::{debug_output, error_output},
};
use crate::{parser::parser::ParseProto, vm::command::Command};
use std::collections::HashMap;
use std::io::Read;

pub struct VirtualMachine {
    stack: Vec<SunPointer>,
    value_map: HashMap<String, SunPointer>,
    meta_map: HashMap<&'static str, SunObject>,
    function_map: HashMap<String, Vec<Command>>,
    is_debug: bool,
    check_global: bool,
    check_stack: bool,
}

impl VirtualMachine {
    pub fn new(is_debug: bool, check_stack: bool, check_global: bool) -> Self {
        let mut vm = VirtualMachine {
            stack: Vec::new(),
            value_map: HashMap::new(),
            meta_map: HashMap::new(),
            function_map: HashMap::new(),
            is_debug,
            check_global,
            check_stack,
        };
        prelude(&mut vm.value_map, &mut vm.meta_map);
        vm
    }

    pub fn run<T: Read>(&mut self, proto: &ParseProto<T>) {
        let mut pc = 1;
        while pc <= proto.commands.len() {
            let command = &proto.commands[pc - 1];
            match &command {
                Command::LoadValue(name) => {
                    let value = self.value_map.get(name).clone();
                    if let Some(value) = value {
                        self.stack.push(value.clone());
                    } else {
                        self.stack.push(SunPointer::new(SunValue::Nil));
                    }
                }
                Command::LoadConst(value) => self.stack.push(SunPointer::new(value.clone())),
                Command::LoadMethod(name) => {
                    if name == "dot" {
                        let self_value = self.stack.pop();
                        match self_value.clone() {
                            Some(p) => {
                                let value = p.get();
                                let value_name = value.get_name();
                                match self.meta_map.get(value_name) {
                                    Some(obj) => match self.stack.pop() {
                                        Some(p) => {
                                            let method_string = p.get();
                                            if let method_name @ SunValue::String(_) = method_string
                                            {
                                                match obj
                                                    .get_method((&method_name).to_string().as_str())
                                                {
                                                    Some(method) => {
                                                        self.stack.push(self_value.unwrap());
                                                        self.stack.push(SunPointer::new(
                                                            SunValue::from(method),
                                                        ));
                                                    }
                                                    None => {
                                                        let e = SunError::AttributeError(format!(
                                                            "failed to find attribute `{}` for type `{}`",
                                                            method_name, value_name
                                                        ));
                                                        error_output(e);
                                                    }
                                                }
                                            } else {
                                                let e = SunError::ParaError(format!(
                                                    "need attribute name but got `{}`",
                                                    method_string
                                                ));
                                                error_output(e);
                                            }
                                        }
                                        None => {
                                            let e = SunError::RunError(format!(
                                                "stack is empty so failed to find attribute"
                                            ));
                                            error_output(e);
                                        }
                                    },
                                    None => {
                                        let e = SunError::TypeError(format!(
                                            "`{value_name}` is not a valid sun type"
                                        ));
                                        error_output(e);
                                    }
                                }
                            }
                            None => {
                                let e = SunError::RunError(format!(
                                    "stack is empty so failed to find object"
                                ));
                                error_output(e);
                            }
                        }
                    } else {
                        match self.stack.last() {
                            Some(p) => {
                                let value = p.get();
                                let value_name = value.get_name();
                                match self.meta_map.get(value_name) {
                                    Some(obj) => match obj.get_method(name) {
                                        Some(method) => {
                                            self.stack.push(SunPointer::new(SunValue::from(method)))
                                        }
                                        None => {
                                            let e = SunError::AttributeError(format!(
                                                "failed to find attribute `{}` for type `{}`",
                                                name, value_name
                                            ));
                                            error_output(e);
                                        }
                                    },
                                    None => {
                                        let e = SunError::TypeError(format!(
                                            "`{value_name}` is not a valid sun type"
                                        ));
                                        error_output(e);
                                    }
                                }
                            }
                            None => {
                                let e = SunError::RunError(format!(
                                    "stack is empty so failed to find object"
                                ));
                                error_output(e);
                            }
                        }
                    }
                }
                Command::StoreGlobal(name) => {
                    self.value_map.insert(
                        name.to_string(),
                        self.stack.pop().unwrap_or(SunPointer::new(SunValue::Nil)),
                    );
                }
                Command::TestJump(jump) => match self.stack.pop() {
                    Some(p) => {
                        if let SunValue::Boolean(false) | SunValue::Nil = p.get() {
                            pc += *jump;
                        }
                    }
                    None => {
                        let e = SunError::RunError(format!(
                            "stack is empty so failed to find get condition"
                        ));
                        error_output(e);
                    }
                },
                Command::Jump(jump) => {
                    pc += jump;
                }
                Command::Back(jump) => {
                    pc -= jump;
                }
                Command::SetTable => {
                    let self_value = self.stack.pop();
                    match self_value {
                        Some(p) => {
                            let mut pp = p.borrow_mut();
                            match self.stack.pop() {
                                Some(value) => {
                                    *pp = value.get();
                                }
                                None => {
                                    let e =
                                        SunError::ParaError(format!("need new value but falied",));
                                    error_output(e);
                                }
                            }
                        }
                        None => {
                            let e = SunError::RunError(format!(
                                "stack is empty so failed to find object"
                            ));
                            error_output(e);
                        }
                    }
                }
                Command::CreateTable(n) => {
                    let mut table = Table::new();
                    for _ in 1..=*n {
                        match self.stack.pop() {
                            Some(p) => {
                                let value = p.get();
                                match value {
                                    SunValue::Table(ref t) => match t.get_by_idx(0) {
                                        Some(p) => {
                                            let content = p.get();
                                            if let s @ SunValue::String(_) = content {
                                                if s == "pair".into() {
                                                    table.extend(t.clone())
                                                }
                                            } else {
                                                table.append(value)
                                            }
                                        }
                                        None => table.append(value),
                                    },
                                    _ => table.append(value),
                                }
                            }
                            None => {
                                let e = SunError::RunError(format!(
                                    "stack is empty so failed to set value for table"
                                ));
                                error_output(e);
                            }
                        }
                    }
                    self.stack.push(SunPointer::new(SunValue::from(table)));
                }
                Command::SetPair(key) => {
                    let mut table = Table::new();
                    match self.stack.pop() {
                        Some(p) => {
                            let value = p.get();
                            table.append(SunValue::from("pair"));
                            table.append_kv(key.to_owned(), value);
                            self.stack.push(SunPointer::new(SunValue::from(table)));
                        }
                        None => {
                            let e = SunError::RunError(format!(
                                "stack is empty so failed to set value for key `{key}`"
                            ));
                            error_output(e);
                        }
                    }
                }
                Command::Call(n) => match self.stack.pop() {
                    Some(p) => {
                        let value = p.get();
                        match value {
                            SunValue::Function(f) => {
                                let mut args = Vec::new();
                                for i in 1..=*n {
                                    if let Some(arg) = self.stack.pop() {
                                        args.push(arg)
                                    } else {
                                        let e = SunError::CallError(format!(
                                            "need `{n}` but provide `{i}` parameters",
                                        ));
                                        error_output(e);
                                    }
                                }
                                let result = f(args);
                                self.stack.extend(result);
                            }
                            other => {
                                let e =
                                    SunError::CallError(format!("`{}` is not a function", other));
                                error_output(e);
                            }
                        }
                    }
                    None => {
                        let e = SunError::RunError(format!(
                            "stack is empty so failed to find function"
                        ));
                        error_output(e);
                    }
                },
            }
            if self.check_stack == true && self.is_debug == false {
                println!();
                debug_output(&self.stack, true);
                println!();
            }
            if self.check_global == true && self.is_debug == false {
                println!();
                debug_output(&self.value_map, true);
                println!()
            }
            if self.is_debug == true {
                println!();
                debug_output(pc, false);
                debug_output(command, false);
                debug_output(&self.stack, true);
                debug_output(&self.value_map, true);
                debug_output(&self.meta_map, true);
                println!();
            }
            pc += 1;
        }
    }
}

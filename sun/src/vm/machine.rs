use crate::{parser::parser::ParseProto, prelude::prelude, vm::command::Command};
use std::collections::HashMap;
use std::io::Read;
use sun_core::{
    container::{Function, SunValue, Table},
    meta::{SunBase, SunMeta},
    utils::{
        log::{debug_output, error_output, log_output, warn_output},
        machine::IsMachine,
        SunError, SunPointer,
    },
};

/// Sun 虚拟机的结构体
pub struct VirtualMachine<'a> {
    /// 调用栈
    stack: Vec<SunPointer>,
    /// 全局变量表
    value_map: HashMap<String, SunPointer>,
    /// 临时变量表
    // temp_map: HashMap<String, SunPointer>,
    /// meta表
    meta_map: HashMap<&'a str, SunMeta>,
    /// 函数表
    // function_map: HashMap<String, Vec<Command>>,
    /// debug 模式标志
    is_debug: bool,
    /// 检查全局变量表标志
    check_global: bool,
    /// 检查调用堆栈标志
    check_stack: bool,
}

impl<'a> VirtualMachine<'a> {
    /// 创建新的虚拟机
    pub fn new(is_debug: bool, check_stack: bool, check_global: bool) -> Self {
        let mut vm = VirtualMachine {
            stack: Vec::new(),
            value_map: HashMap::new(),
            // temp_map: HashMap::new(),
            meta_map: HashMap::new(),
            // function_map: HashMap::new(),
            is_debug,
            check_global,
            check_stack,
        };
        // 预导入的模块
        prelude(&mut vm.value_map, &mut vm.meta_map);
        vm
    }

    /// 运行虚拟机 `proto`: 语法分析器
    pub fn run<T: Read>(&mut self, proto: &ParseProto<T>) {
        let mut pc = 1;
        while pc <= proto.commands.len() {
            let command = &proto.commands[pc - 1];
            match &command {
                Command::LoadValue(name) => {
                    let value = self.value_map.get(name.as_str()).clone();
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
                                let meta_name = value.get_name();
                                match self.stack.pop() {
                                    Some(method_name) => match method_name.get() {
                                        ref method_name @ SunValue::String(_) => {
                                            let method_name: String = method_name.into();
                                            let method =
                                                self.get_method(meta_name, method_name.as_str());
                                            self.stack.push(self_value.unwrap());
                                            self.stack.push(SunPointer::new(SunValue::from(method)))
                                        }
                                        other => {
                                            let e = SunError::ParaError(format!(
                                                "expect attribute name but got `{other}`"
                                            ));
                                            error_output(e);
                                        }
                                    },
                                    None => {
                                        let e = SunError::RunError(format!(
                                            "stack is empty so failed to find object"
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
                                let meta_name = value.get_name();
                                let method = self.get_method(meta_name, name);
                                self.stack.push(SunPointer::new(SunValue::from(method)));
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
                Command::LoadMetamethod(meta_name, method_name) => {
                    let method = self.get_method(meta_name, method_name);
                    self.stack.push(SunPointer::new(SunValue::from(method)));
                }
                Command::StoreGlobal(name) => {
                    match self.stack.pop() {
                        Some(value) => match value.get() {
                            SunValue::Nil => {
                                warn_output("Nil value will not be insert into global value map")
                            }
                            _ => {
                                self.value_map.insert(name.to_string(), value);
                            }
                        },
                        None => warn_output("Nil value will not be insert into global value map"),
                    };
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
                            SunValue::Function(f) => match f {
                                Function::RustFunction(rf) => {
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
                                    let res = rf(args);
                                    self.stack.extend(res);
                                }
                                Function::SysFunction(sf) => sf(self),
                            },
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
                Command::Import(lib_name) => self.include(lib_name),
            }
            self.debug(pc, &command);
            pc += 1;
        }
    }

    /// debug 模式的打印信息处理 `pc`: 程序计数器 `command`: 当前运行的指令
    fn debug(&self, pc: usize, command: &Command) {
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
    }

    /// 递归查找基类的方法
    fn get_method(&self, meta_name: &str, method_name: &str) -> Function {
        match self.meta_map.get(meta_name) {
            Some(meta) => match meta.get_method(method_name) {
                Some(method) => method,
                None => match meta.get_base() {
                    &SunBase::None => {
                        let e = SunError::AttributeError(format!(
                            "failed to find attribute `{}` for type `{}`",
                            method_name, meta_name
                        ));
                        error_output(e);
                    }
                    &SunBase::Object => self.get_method("Object", method_name),
                    &SunBase::Other(ref c) => self.get_method(c, method_name),
                },
            },
            None => {
                let e = SunError::TypeError(format!("`{meta_name}` is not a valid sun type"));
                error_output(e);
            }
        }
    }

    fn include(&mut self, lib_name: &str) {
        dbg!(lib_name);
    }
}

impl<'a> IsMachine for VirtualMachine<'a> {
    fn pop(&mut self) -> Option<SunPointer> {
        self.stack.pop()
    }

    fn drop(&mut self, name: &str) {
        self.value_map.remove(name);
    }

    fn get_meta(&self, name: &str) -> Option<Vec<&str>> {
        if let Some(meta) = self.meta_map.get(name) {
            Some(meta.get_methods())
        } else {
            None
        }
    }

    fn show_global(&self) {
        let variables = self
            .value_map
            .iter()
            .filter(|(_, p)| p.get().get_name() != "function")
            .map(|(name, value)| format!("{name}: {value:?}"))
            .collect::<Vec<String>>()
            .join(",\n");
        log_output(variables)
    }
}

use crate::{
    add_prelude_methods, add_prelude_sys_methods,
    value::{
        sun_boolean::SunBoolean, sun_function::SunFunction, sun_nil::SunNil, sun_number::SunNumber,
        sun_table::SunTable,
    },
};
use std::{collections::HashMap, process};
use sun_core::{
    add_metas,
    container::{Function, RustFunction, SunValue, SysFunction},
    meta::SunMeta,
    utils::{
        log::{error_output, log_output, warn_output},
        object::_type,
        IsMachine, SunError, SunObject, SunPointer,
    },
};

/// 预导入
pub fn prelude(
    value_map: &mut HashMap<String, SunPointer>,
    meta_map: &mut HashMap<&'static str, SunMeta>,
) {
    add_metas!(
        meta_map,
        ("Object", SunObject),
        ("Nil", SunNil),
        ("Bool", SunBoolean),
        ("Number", SunNumber),
        ("Table", SunTable),
        ("Function", SunFunction)
    );
    add_prelude_methods!(value_map, print, exit);
    add_prelude_sys_methods!(value_map, drop, show);
    value_map.insert("type".to_string(), SunPointer::new(SunValue::from(_type())));
}

/// 批量添加预导入方法
#[macro_export]
macro_rules! add_prelude_methods {
    ($map:expr, $($func:ident),+) => {
        $(
            let name = stringify!($func);
            $map.insert(name.to_string(), SunPointer::new(SunValue::from(Function::from($func as RustFunction))));
        )+
    };
}

/// 批量添加预导入系统方法
#[macro_export]
macro_rules! add_prelude_sys_methods {
    ($map:expr, $($func:ident),+) => {
        $(
            let name = stringify!($func);
            $map.insert(name.to_string(), SunPointer::new(SunValue::from(Function::from($func as SysFunction))));
        )+
    };
}

/// 打印变量
fn print(args: Vec<SunPointer>) -> Vec<SunPointer> {
    let log = args
        .iter()
        .map(|arg| arg.get().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    log_output(log);
    Vec::new()
}

/// 退出程序
fn exit(_: Vec<SunPointer>) -> Vec<SunPointer> {
    process::exit(0)
}

/// 删除全局变量
fn drop(vm: &mut dyn IsMachine) {
    match vm.pop() {
        Some(p) => match p.get() {
            name @ SunValue::String(_) => {
                vm.drop((&name).to_string().as_str());
            }
            other => {
                let e = SunError::ParaError(format!("need variable name but got `{}`", other));
                error_output(e);
            }
        },
        None => {
            let e = SunError::RunError(format!("stack is empty so failed to find attribute"));
            error_output(e);
        }
    }
}

/// 显示全局变量
fn show(vm: &mut dyn IsMachine) {
    match vm.pop() {
        Some(p) => match p.get() {
            para @ SunValue::String(_) => match (&para).to_string().as_str() {
                "global" => vm.show_global(),
                other => warn_output(format!("invalid 'show' parameter `{other}`")),
            },
            other => {
                let e = SunError::ParaError(format!("need 'show' parameter but got `{}`", other));
                error_output(e);
            }
        },
        None => warn_output(format!("failed to got 'show' parameter")),
    }
}

use crate::{
    container::{Function, RustFunction, SysFunction},
    meta::{OwnSunMeta, SunMeta},
    utils::{
        log::{error_output, log_output},
        IsMachine, SunError, SunPointer,
    },
};
use std::collections::HashMap;

/// `Object` 元数据
#[derive(Debug, Clone)]
pub struct SunObject {
    pub meta: SunMeta,
}

impl SunObject {
    /// 新建新的 `Object` 元数据
    pub fn new(name: &'static str) -> SunObject {
        let meta = SunMeta::new(name, HashMap::new());
        let mut obj = SunObject { meta };
        obj.set_method("type", _type());
        obj.set_method("clone", clone());
        obj.set_method("meta", Function::from(get_meta as SysFunction));
        obj
    }

    pub fn get_meta(&self) -> SunMeta {
        self.meta.clone()
    }
}

/// 获取类型名的类型方法
pub fn _type() -> Function {
    let f = |value: Vec<SunPointer>| -> Vec<SunPointer> {
        let value = value[0].get();
        log_output(value.get_name());
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 获取数据的拷贝
pub fn clone() -> Function {
    let f = |value: Vec<SunPointer>| -> Vec<SunPointer> {
        let res = value[0].deep_copy();
        vec![res]
    };
    Function::from(f as RustFunction)
}

/// 获取类型的元数据的类型方法
pub fn get_meta(vm: &mut dyn IsMachine) {
    match vm.pop() {
        Some(p) => match p.get() {
            value => {
                if let Some(res) = vm.get_meta(value.get_name()) {
                    let res = res.join(", ");
                    log_output(res)
                } else {
                    let e = SunError::TypeError(format!(
                        "`{}` is not a valid sun type",
                        value.get_name()
                    ));
                    error_output(e);
                }
            }
        },
        None => {
            let e = SunError::RunError(format!("stack is empty so failed to find attribute"));
            error_output(e);
        }
    }
}

/// 继承自 Object 的应当实现的特征
pub trait IsSunObject {
    /// 获取 Object
    fn get_obj(&self) -> &SunObject;
    /// 获取 Object 的可变引用
    fn get_mut_obj(&mut self) -> &mut SunObject;
}

impl OwnSunMeta for SunObject {
    fn get_method(&self, key: &str) -> Option<Function> {
        self.meta.get_method(key)
    }

    fn set_method(&mut self, key: &str, value: Function) {
        self.meta.set_method(key, value)
    }

    fn get_methods(&self) -> Vec<&str> {
        self.meta.get_methods()
    }

    fn get_name(&self) -> &str {
        self.meta.get_name()
    }
}

/// 批量添加元方法
#[macro_export]
macro_rules! add_methods {
    ($obj:expr, $type_name:ty, $(($name:expr, $method:ident)),+) => {
        $(
            $obj.set_method($name, <$type_name>::$method());
        )+
    };
}

/// 批量添加类型方法
#[macro_export]
macro_rules! add_type_methods {
    ($obj:expr, $(($name:expr, $method:ident)),+) => {
        $(
            $obj.get_mut_obj().set_method($name, $method());
        )+
    };
}

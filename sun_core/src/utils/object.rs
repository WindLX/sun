use crate::{
    add_methods,
    container::{Function, RustFunction, SysFunction},
    meta::{OwnSunMeta, SunBase, SunMeta},
    utils::{
        log::{error_output, log_output},
        IsMachine, SunError, SunPointer,
    },
};

/// `SunObject` 元数据
#[derive(Debug, Clone)]
pub struct SunObject {
    meta: SunMeta,
}

impl SunObject {
    /// 新建新的 `Object` 元数据
    pub fn new() -> SunObject {
        let mut meta = SunMeta::new("Object", SunBase::None);
        add_methods!(meta, ("type", _type), ("clone", clone), ("meta", get_meta));
        SunObject { meta }
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
pub fn get_meta() -> Function {
    let f = |vm: &mut dyn IsMachine| match vm.pop() {
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
    };
    Function::from(f as SysFunction)
}

impl OwnSunMeta for SunObject {
    fn get_meta(&self) -> &SunMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut SunMeta {
        &mut self.meta
    }
}

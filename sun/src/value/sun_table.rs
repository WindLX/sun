use colorized::*;
use sun_core::{
    add_methods,
    container::{Function, RustFunction, SunValue},
    meta::{meta_methods::container::IndexAble, OwnSunMeta},
    utils::{
        log::{error_output, warn_output},
        IsSunObject, SunError, SunObject, SunPointer,
    },
};

/// `Table` 类型的元数据
#[derive(Clone, Debug)]
pub struct SunTable {
    obj: SunObject,
}

impl IsSunObject for SunTable {
    fn get_obj(&self) -> &SunObject {
        &self.obj
    }

    fn get_mut_obj(&mut self) -> &mut SunObject {
        &mut self.obj
    }
}

impl SunTable {
    /// 新建 `Table` 元数据
    pub fn new() -> SunTable {
        let mut obj = SunObject::new("table");
        add_methods!(obj, SunTable, ("index", index));
        obj.set_method("remove", remove());
        obj.set_method("push", push());
        obj.set_method("insert", insert());
        obj.set_method("extend", extend());
        obj.set_method("aextend", aextend());
        obj.set_method("dextend", dextend());
        obj.set_method("alen", alen());
        obj.set_method("dlen", dlen());
        obj.set_method("len", len());
        obj.set_method("clone", clone());
        SunTable { obj }
    }
}

/// 重写自身的深拷贝
fn clone() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args[0].deep_copy();
        let p = p.borrow();
        if let SunValue::Table(t) = &*p {
            vec![SunPointer::new(SunValue::from(t.deep_copy()))]
        } else {
            vec![]
        }
    };
    Function::from(f as RustFunction)
}

/// 从 `Table` 中按索引和键移除多个值
fn remove() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut res = Vec::new();
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            for arg in args {
                match arg.get() {
                    key @ SunValue::String(_) => match t.remove_by_key((&key).to_string().as_str())
                    {
                        Some(p) => res.push(p),
                        None => {
                            warn_output(
                                format!("failed to find target value by key `{key}` so the table will not be changed")
                                    .color(Colors::YellowFg),
                            );
                        }
                    },
                    SunValue::Number(idx) => {
                        if idx.fract() != 0.0 {
                            warn_output(
                                format!("index is not an integer so there may be problems")
                                    .color(Colors::YellowFg),
                            );
                        }
                        match t.remove_by_idx(idx as usize) {
                            Some(p) => res.push(p),
                            None => {
                                warn_output(
                                    format!("failed to find target value by index `{idx}` so the table will not be changed")
                                        .color(Colors::YellowFg),
                                );
                            }
                        }
                    }
                    other => {
                        let e = SunError::ParaError(format!("invalid key or index `{other}`"));
                        error_output(e);
                    }
                }
            }
        }
        res
    };
    Function::from(f as RustFunction)
}

/// 向 `Table` 的数组中追加多个值
fn push() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            for arg in args {
                let value = arg.get();
                t.append(value)
            }
        }
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 向 `Table` 中插入新值
fn insert() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 2 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            match (args[0].get(), args[1].get()) {
                (key @ SunValue::String(_), value) => {
                    if let Some(_) = t.get_by_key((&key).to_string().as_str()) {
                        warn_output(
                            format!("key `{key}` already exists so the value will be changed")
                                .color(Colors::YellowFg),
                        );
                    }
                    t.append_kv((&key).into(), value)
                }
                (SunValue::Number(index), value) => {
                    if index < 0.0 {
                        let e = SunError::ParaError(format!("negative can't be index"));
                        error_output(e);
                    }
                    if index.fract() != 0.0 {
                        warn_output(format!("parameter is not an integer so it's decimal part will be truncated as an index").color(Colors::YellowFg));
                    }
                    t.insert(index as usize, value)
                }
                (other, _) => {
                    let e = SunError::KeyError(format!("invalid key `{other}`"));
                    error_output(e);
                }
            }
        }
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 合并两个 `Table`
fn extend() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            match args[0].get() {
                SunValue::Table(t2) => {
                    t.extend(t2.clone());
                    t.extend_array(t2.clone())
                }
                other => {
                    let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                    error_output(e);
                }
            }
        }
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 合并两个 `Table` 的数组部分
fn aextend() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            match args[0].get() {
                SunValue::Table(t2) => t.extend_array(t2.clone()),
                other => {
                    let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                    error_output(e);
                }
            }
        }
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 合并两个 `Table` 的字典部分
fn dextend() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            match args[0].get() {
                SunValue::Table(t2) => {
                    t.extend(t2.clone());
                }
                other => {
                    let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                    error_output(e);
                }
            }
        }
        vec![]
    };
    Function::from(f as RustFunction)
}

/// 获取 `Table` 数组部分长度
fn alen() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args[0].get();
        match p {
            SunValue::Table(t) => vec![t.alen()],
            other => {
                let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                error_output(e);
            }
        }
    };
    Function::from(f as RustFunction)
}

/// 获取 `Table` 字典部分长度
fn dlen() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args[0].get();
        match p {
            SunValue::Table(t) => vec![t.dlen()],
            other => {
                let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                error_output(e);
            }
        }
    };
    Function::from(f as RustFunction)
}

/// 获取 `Table` 总长度
fn len() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::ParaError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args[0].get();
        match p {
            SunValue::Table(t) => vec![t.len()],
            other => {
                let e = SunError::ParaError(format!("expect `table` but got `{other}`"));
                error_output(e);
            }
        }
    };
    Function::from(f as RustFunction)
}

impl IndexAble for SunTable {
    fn index() -> Function {
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let res = match (arg_0, arg_1) {
                (SunValue::Table(t), SunValue::Number(idx)) => {
                    if idx.fract() != 0.0 {
                        warn_output(
                            format!("index is not an integer so there may be problems")
                                .color(Colors::YellowFg),
                        );
                    }
                    if let Some(res) = t.get_by_idx(idx as usize) {
                        vec![res]
                    } else {
                        let e =
                            SunError::IndexError(format!("failed to get value by index `{idx}`"));
                        error_output(e);
                    }
                }
                (SunValue::Table(t), key @ SunValue::String(_)) => {
                    if let Some(res) = t.get_by_key((&key).to_string().as_str()) {
                        vec![res]
                    } else {
                        let e = SunError::KeyError(format!("failed to get value by key `{key}`"));
                        error_output(e);
                    }
                }
                _ => {
                    let e = SunError::ParaError(format!("invalid parameters for get value"));
                    error_output(e);
                }
            };
            res
        };
        Function::from(f as RustFunction)
    }
}

use super::super::{
    sun_function::Function,
    sun_meta::{container::IndexAble, OwnSunMeta},
    sun_object::{IsSunObject, SunObject, SunValue},
    sun_pointer::SunPointer,
};
use crate::add_methods;
use crate::utils::{
    err::SunError,
    log::{error_output, warn_output},
};
use colorized::*;
use std::collections::HashMap;
use std::fmt;

/// `Table` 类型的数据容器
#[derive(Clone)]
pub struct Table {
    array: Vec<SunPointer>,
    dict: HashMap<String, SunPointer>,
}

impl Table {
    /// 新建新的 `Table` 容器
    pub fn new() -> Self {
        Table {
            array: Vec::new(),
            dict: HashMap::new(),
        }
    }

    /// 向数组添加新值
    pub fn append(&mut self, value: SunValue) {
        self.array.push(SunPointer::new(value))
    }

    /// 向数组中指定索引处插入新值
    pub fn insert(&mut self, index: usize, value: SunValue) {
        self.array.insert(index, SunPointer::new(value))
    }

    /// 向字典添加新键值对
    pub fn append_kv(&mut self, key: String, value: SunValue) {
        self.dict.insert(key, SunPointer::new(value));
    }

    /// 按索引获取内容的指针，引用计数增加
    pub fn get_by_idx(&self, idx: usize) -> Option<SunPointer> {
        match self.array.get(idx) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// 按键获取内容的指针，引用计数增加
    pub fn get_by_key(&self, key: &str) -> Option<SunPointer> {
        match self.dict.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// 按索引移除内容的指针
    pub fn remove_by_idx(&mut self, idx: usize) -> Option<SunPointer> {
        if self.array.len() <= idx {
            None
        } else {
            Some(self.array.remove(idx))
        }
    }

    /// 按键移除内容的指针
    pub fn remove_by_key(&mut self, key: &str) -> Option<SunPointer> {
        self.dict.remove(key)
    }

    /// 合并两个 `Table` 的字典
    pub fn extend(&mut self, other: Table) {
        self.dict.extend(other.dict)
    }

    /// 合并两个 `Table` 的数组
    pub fn extend_array(&mut self, other: Table) {
        self.array.extend(other.array)
    }

    /// 获取数组长度
    pub fn alen(&self) -> SunPointer {
        let n = self.array.len();
        SunPointer::new(SunValue::from(n as f64))
    }

    /// 获取字典长度
    pub fn dlen(&self) -> SunPointer {
        let n = self.dict.len();
        SunPointer::new(SunValue::from(n as f64))
    }

    /// 获取总长度
    pub fn len(&self) -> SunPointer {
        let n = self.dict.len() + self.array.len();
        SunPointer::new(SunValue::from(n as f64))
    }

    /// 自身的深拷贝
    pub fn deep_copy(&self) -> Self {
        let array = self.array.iter().map(|p| p.deep_copy()).collect();
        let dict = self
            .dict
            .iter()
            .map(|(k, p)| (k.clone(), p.deep_copy()))
            .collect();
        Table { array, dict }
    }
}

/// `Table` 类型的元数据
#[derive(Clone, Debug)]
pub struct SunTable {
    obj: SunObject,
}

impl IsSunObject for SunTable {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
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
    f
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
    f
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
    f
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
    f
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
    f
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
    f
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
    f
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
    f
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
    f
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
    f
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
        f
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(
            f,
            "{}: {}",
            "Array".color(Colors::YellowFg),
            self.array.len()
        )?;
        write!(f, "{:<10} {:<10}\n", "Index", "Value")?;
        for (i, item) in self.array.iter().enumerate() {
            write!(f, "{:<10} {:<10?}\n", i, item)?;
        }
        writeln!(
            f,
            "{}:  {}",
            "Dict".color(Colors::YellowFg),
            self.array.len()
        )?;
        write!(f, "{:<10} {:<10}\n", "Key", "Value")?;
        for (key, value) in self.dict.iter() {
            write!(f, "{:<10} {:<10?}\n", key, value)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "Array:")?;
        write!(f, "{:<10} {:<10}\n", "Index", "Value")?;
        for (i, item) in self.array.iter().enumerate() {
            write!(f, "{:<10} {:<10?}\n", i, item)?;
        }

        writeln!(f, "Dict:")?;
        write!(f, "{:<10} {:<10}\n", "Key", "Value")?;
        for (key, value) in self.dict.iter() {
            write!(f, "{:<15} {:<10?}\n", key, value)?;
        }

        Ok(())
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.array.len() != other.array.len() || self.dict.len() != other.dict.len() {
            return false;
        }

        for (i, value) in self.array.iter().enumerate() {
            if !value.eq(&other.array[i]) {
                return false;
            }
        }

        for (key, value) in &self.dict {
            if !value.eq(other
                .dict
                .get(key)
                .unwrap_or(&SunPointer::new(SunValue::Nil)))
            {
                return false;
            }
        }

        true
    }
}

impl Eq for Table {}

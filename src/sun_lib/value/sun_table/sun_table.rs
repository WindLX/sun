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

#[derive(Clone)]
pub struct Table {
    array: Vec<SunPointer>,
    dict: HashMap<String, SunPointer>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            array: Vec::new(),
            dict: HashMap::new(),
        }
    }

    pub fn append(&mut self, value: SunValue) {
        self.array.push(SunPointer::new(value))
    }

    pub fn append_kv(&mut self, key: String, value: SunValue) {
        self.dict.insert(key, SunPointer::new(value));
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<SunPointer> {
        match self.array.get(idx) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn get_by_key(&self, key: &str) -> Option<SunPointer> {
        match self.dict.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn remove_by_idx(&mut self, idx: usize) -> Option<SunPointer> {
        if self.array.len() <= idx {
            None
        } else {
            Some(self.array.remove(idx))
        }
    }

    pub fn remove_by_key(&mut self, key: &str) -> Option<SunPointer> {
        self.dict.remove(key)
    }

    pub fn extend(&mut self, other: Table) {
        self.dict.extend(other.dict)
    }

    pub fn extend_array(&mut self, other: Table) {
        self.array.extend(other.array)
    }

    pub fn alen(&self) -> SunPointer {
        let n = self.array.len();
        SunPointer::new(SunValue::from(n as f64))
    }

    pub fn dlen(&self) -> SunPointer {
        let n = self.dict.len();
        SunPointer::new(SunValue::from(n as f64))
    }

    pub fn len(&self) -> SunPointer {
        let n = self.dict.len() + self.array.len();
        SunPointer::new(SunValue::from(n as f64))
    }
}

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
    pub fn new() -> SunTable {
        let mut obj = SunObject::new("table");
        add_methods!(obj, SunTable, ("index", index));
        obj.set_method("remove", remove());
        obj.set_method("push", push());
        obj.set_method("insert", insert());
        obj.set_method("extend", extend());
        obj.set_method("alen", alen());
        obj.set_method("dlen", dlen());
        obj.set_method("len", len());
        SunTable { obj }
    }
}

fn remove() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut res = Vec::new();
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            for arg in args {
                match arg.get() {
                    SunValue::String(ref key) => match t.remove_by_key(key) {
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

fn push() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
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

fn insert() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 2 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
                error_output(e);
            }
        }
        let p = args.remove(0);
        let mut t = p.borrow_mut();
        if let SunValue::Table(t) = &mut *t {
            match (args[0].get(), args[1].get()) {
                (SunValue::String(key), value) => {
                    if let Some(_) = t.get_by_key(&key) {
                        warn_output(
                            format!("key `{key}` already exists so the value will be changed")
                                .color(Colors::YellowFg),
                        );
                    }
                    t.append_kv(key, value)
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

fn extend() -> Function {
    let f = |mut args: Vec<SunPointer>| {
        if args.len() <= 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
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

fn alen() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
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

fn dlen() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
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

fn len() -> Function {
    let f = |args: Vec<SunPointer>| {
        if args.len() < 1 {
            {
                let e = SunError::CallError(format!("the number of parameters is too few"));
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
                (SunValue::Table(t), SunValue::String(key)) => {
                    if let Some(res) = t.get_by_key(key.as_str()) {
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

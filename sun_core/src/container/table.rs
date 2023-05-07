use crate::{container::SunValue, sunc::sun_struct::*, utils::SunPointer};
use colorized::*;
use std::collections::HashMap;
use std::ffi::CStr;
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

    pub fn get_dict(&self) -> HashMap<String, SunPointer> {
        self.dict.clone()
    }

    pub fn get_array(&self) -> Vec<SunPointer> {
        self.array.clone()
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

impl From<TableC> for Table {
    fn from(value: TableC) -> Self {
        let mut array = Vec::new();
        let mut dict = HashMap::new();
        for i in 0..value.array_len {
            let p = unsafe { &*value.array.offset(i as isize) };
            let v = unsafe { &*(*p).data };
            array.push(SunPointer::new(v.clone().into()))
        }
        for j in 0..value.dict_len {
            let p = unsafe { &*value.dict.offset(j as isize) };
            let v = unsafe { &*(*(*p).pointer).data };
            let k = unsafe { CStr::from_ptr((*p).key).to_string_lossy().into_owned() };
            dict.insert(k, SunPointer::new(v.clone().into()));
        }
        Table { array, dict }
    }
}

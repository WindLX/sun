use crate::utils::SunPointer;
use colorized::*;
use std::collections::HashMap;
use std::fmt;

/// `Class` 类型的数据容器
#[derive(Clone)]
pub struct Class {
    name: String,
    attributes: HashMap<String, SunPointer>,
}

impl Class {
    /// 创建新的类数据 容器
    pub fn new(name: &str, attributes: HashMap<String, SunPointer>) -> Self {
        Class {
            name: name.to_string(),
            attributes,
        }
    }

    /// 获取类名
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// 设置属性
    pub fn set_attribute(&mut self, attr_name: &str, attribute: SunPointer) {
        self.attributes.insert(attr_name.to_string(), attribute);
    }

    /// 移除属性
    pub fn remove_attribute(&mut self, attr_name: &str) {
        self.attributes.remove(attr_name);
    }

    /// 获取属性
    pub fn get_attribute(&self, attr_name: &str) -> Option<SunPointer> {
        self.attributes.get(attr_name).cloned()
    }
}

pub trait IsSunClass {
    fn get_class(&self) -> &Class;
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        let str = format!("{}:", self.name).color(Colors::YellowFg);
        writeln!(f, "{}", str)?;
        for (key, value) in self.attributes.iter() {
            write!(f, "{:<10} {:<10?}\n", key, value)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        let str = format!("{}:", self.name).color(Colors::YellowFg);
        writeln!(f, "{}:", str)?;
        for (key, value) in self.attributes.iter() {
            write!(f, "{:<10} {:<10?}\n", key, value)?;
        }
        Ok(())
    }
}

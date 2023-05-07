use crate::{sunc::sun_struct::*, utils::SunPointer};
use std::collections::HashMap;
use std::ffi::CStr;

/// `Class` 类型的数据容器
#[derive(Debug, Clone)]
pub struct Class {
    name: &'static str,
    attributes: HashMap<String, SunPointer>,
}

impl Class {
    /// 创建新的类数据 容器
    pub fn new(name: &'static str, attributes: HashMap<String, SunPointer>) -> Self {
        Class { name, attributes }
    }

    /// 获取类名
    pub fn get_name(&self) -> &str {
        self.name
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

    pub fn get_all(&self) -> HashMap<String, SunPointer> {
        self.attributes.clone()
    }
}

impl From<ClassC> for Class {
    fn from(value: ClassC) -> Self {
        let name = unsafe { CStr::from_ptr(value.name).to_string_lossy().into_owned() };
        let name: &'static str = Box::leak(name.into_boxed_str());
        let mut attr = HashMap::new();
        for j in 0..value.attr_len {
            let p = unsafe { &*value.attributes.offset(j as isize) };
            let v = unsafe { &*(*(*p).pointer).data };
            let k = unsafe { CStr::from_ptr((*p).key).to_string_lossy().into_owned() };
            attr.insert(k, SunPointer::new(v.clone().into()));
        }
        Class {
            name,
            attributes: attr,
        }
    }
}

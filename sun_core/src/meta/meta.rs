use crate::container::function::Function;
use std::collections::HashMap;

/// `SunBase` 基类
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SunBase {
    Other(String),
    Object,
}

/// Sun 的元信息，定义类型的元数据和应当拥有的行为
#[derive(Debug, Clone)]
pub struct SunMeta {
    /// 类型名
    name: String,
    /// 基类
    base: SunBase,
    /// 类型方法表
    methods: HashMap<String, Function>,
}

impl SunMeta {
    /// 创建新的类型元信息
    pub fn new(name: &str, base: SunBase) -> Self {
        SunMeta {
            name: name.to_string(),
            base,
            methods: HashMap::new(),
        }
    }

    /// 以方法名从类型方法表查找方法的指针
    pub fn get_method(&self, key: &str) -> Option<Function> {
        self.methods.get(key).cloned()
    }

    /// 设置新的类型方法
    pub fn set_method(&mut self, key: &str, value: Function) {
        self.methods.insert(key.to_string(), value);
    }

    /// 获取所有的方法
    pub fn get_methods(&self) -> Vec<&str> {
        self.methods.iter().map(|(key, _)| key.as_str()).collect()
    }

    /// 获取类型名
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// 获取基类
    pub fn get_base(&self) -> &SunBase {
        &self.base
    }
}

/// 拥有元数据的类型特征
pub trait OwnSunMeta {
    /// 获取元数据的引用
    fn get_meta(&self) -> &SunMeta;
    /// 获取元数据的可变引用
    fn get_meta_mut(&mut self) -> &mut SunMeta;
}

/// 批量添加类型元信息
#[macro_export]
macro_rules! add_metas {
    ($map:expr, $(($name:expr, $meta:ty)),+) => {
        use sun_core::meta::OwnSunMeta;
        $(
            $map.insert($name, <$meta>::new().get_meta().clone());
        )+
    };
}

/// 批量添加元方法
#[macro_export]
macro_rules! add_meta_methods {
    ($obj:expr, $type_name:ty, $(($name:expr, $method:ident)),+) => {
        $(
            $obj.set_method($name, <$type_name>::$method());
        )+
    };
}

/// 批量添加方法
#[macro_export]
macro_rules! add_methods{
    ($obj:expr, $(($name:expr, $method:ident)),+) => {
        $(
            $obj.set_method($name, $method());
        )+
    };
}

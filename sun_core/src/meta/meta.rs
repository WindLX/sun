use crate::container::function::Function;
use std::collections::HashMap;

/// Sun 的元信息，定义类型的元数据和应当拥有的行为
#[derive(Debug, Clone)]
pub struct SunMeta {
    /// 类型名
    pub name: &'static str,
    /// 类型方法表
    pub methods: HashMap<String, Function>,
}

impl SunMeta {
    /// 创建新的类型元信息
    pub fn new(name: &'static str, methods: HashMap<String, Function>) -> Self {
        SunMeta { name, methods }
    }

    /// 获取类型名
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_all(&self) -> HashMap<String, Function> {
        self.methods.clone()
    }
}

impl OwnSunMeta for SunMeta {
    fn get_method(&self, key: &str) -> Option<Function> {
        self.methods.get(key).cloned()
    }

    fn set_method(&mut self, key: &str, value: Function) {
        self.methods.insert(key.to_string(), value);
    }

    fn get_methods(&self) -> Vec<&str> {
        self.methods.iter().map(|(key, _)| key.as_str()).collect()
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

/// 拥有元信息的类型特征
pub trait OwnSunMeta {
    /// 以方法名从类型方法表查找方法的指针
    fn get_method(&self, key: &str) -> Option<Function>;
    /// 设置新的类型方法
    fn set_method(&mut self, key: &str, value: Function);
    /// 获取所有的方法
    fn get_methods(&self) -> Vec<&str>;
    /// 获取类型名
    fn get_name(&self) -> &str;
}

/// 批量添加类型元信息
#[macro_export]
macro_rules! add_metas {
    ($map:expr, $(($name:expr, $meta:ty)),+) => {
        $(
            $map.insert($name, <$meta>::new().get_obj().clone());
        )+
    };
}

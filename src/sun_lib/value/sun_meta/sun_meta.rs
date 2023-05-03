use super::super::sun_function::Function;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SunMeta {
    name: &'static str,
    methods: HashMap<String, Function>,
}

impl SunMeta {
    pub fn new(name: &'static str, methods: HashMap<String, Function>) -> Self {
        SunMeta { name, methods }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl OwnSunMeta for SunMeta {
    fn get_method(&self, key: &str) -> Option<Function> {
        self.methods.get(key).cloned()
    }

    fn set_method(&mut self, key: &str, value: Function) {
        self.methods.insert(key.to_string(), value);
    }
}

pub trait OwnSunMeta {
    fn get_method(&self, key: &str) -> Option<Function>;
    fn set_method(&mut self, key: &str, value: Function);
}

#[macro_export]
macro_rules! add_metas {
    ($map:expr, $(($name:expr, $meta:ty)),+) => {
        $(
            $map.insert($name, <$meta>::new().get_obj());
        )+
    };
}

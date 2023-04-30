use crate::sun_lib::{sun_value::SunValue, table::Table};
use std::collections::HashMap;

pub struct SunMod {
    name: String,
    doc: HashMap<String, SunValue>,
    consts: HashMap<String, SunValue>,
    func: HashMap<String, SunValue>,
}

impl SunMod {
    pub fn new(
        name: String,
        doc: HashMap<String, SunValue>,
        consts: HashMap<String, SunValue>,
        func: HashMap<String, SunValue>,
    ) -> Self {
        SunMod {
            name,
            doc,
            consts,
            func,
        }
    }

    pub fn trans_2_tuple(&self) -> (String, Table) {
        let mut mod_table = Table::new();
        let mut doc = Table::new();
        doc.copy_dict(self.doc.clone());
        let mut consts = Table::new();
        consts.copy_dict(self.consts.clone());
        let mut func = Table::new();
        func.copy_dict(self.func.clone());
        mod_table.insert_kv("doc".to_string(), SunValue::from(doc));
        mod_table.insert_kv("consts".to_string(), SunValue::from(consts));
        mod_table.insert_kv("func".to_string(), SunValue::from(func));
        (self.name.clone(), mod_table)
    }
}

pub trait Includable {
    fn generate_mod(&self) -> SunMod;
    fn include(&self, global_map: &mut HashMap<String, SunValue>) {
        let sun_mod = self.generate_mod();
        let (k, v) = sun_mod.trans_2_tuple();
        global_map.insert(k, SunValue::from(v));
    }
}

pub trait Preludable {
    fn generate_preclude_mod(&self) -> HashMap<String, SunValue>;
    fn prelude(&self, global_map: &mut HashMap<String, SunValue>) {
        let sun_mod = self.generate_preclude_mod();
        sun_mod.into_iter().for_each(move |(k, v)| {
            global_map.insert(k, SunValue::from(v));
        });
    }
}

#[macro_export]
macro_rules! funcs {
    ( $( $func_name:expr),* ) => {
        {
            use crate::sun_lib::sun_value::SunFunc;
            use std::collections::HashMap;
            let mut funcs: HashMap<String, SunValue> = HashMap::new();
            $(
                funcs.insert(stringify!($func_name).to_string(), SunValue::from($func_name as SunFunc));
            )*
            funcs
        }
    };
}

#[macro_export]
macro_rules! funcs_none {
    (  ) => {{
        use std::collections::HashMap;
        let funcs: HashMap<String, SunValue> = HashMap::new();
        funcs
    }};
}

#[macro_export]
macro_rules! consts {
    ( $( $const_name:expr),* ) => {
        {
            use std::collections::HashMap;
            let mut consts: HashMap<String, SunValue> = HashMap::new();
            $(
                consts.insert(stringify!($const_name).to_string().to_lowercase(), SunValue::from($const_name));
            )*
            consts
        }
    };
}

#[macro_export]
macro_rules! consts_none {
    (  ) => {{
        use std::collections::HashMap;
        let consts: HashMap<String, SunValue> = HashMap::new();
        consts
    }};
}

#[macro_export]
macro_rules! docs {
    ( $($doc_path:expr),* ) => {{
        use std::collections::HashMap;
        let mut docs: HashMap<String, SunValue> = HashMap::new();
        docs
    }};
}

#[macro_export]
macro_rules! docs_none {
    (  ) => {{
        use std::collections::HashMap;
        let docs: HashMap<String, SunValue> = HashMap::new();
        docs
    }};
}

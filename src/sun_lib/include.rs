use crate::vm::value::SunValue;
use std::collections::HashMap;

pub trait Include {
    fn include(global_map: &mut HashMap<String, SunValue>);
}

#[macro_export]
macro_rules! include_func {
    ( $map:expr, $( $func_name:ident ),+ ) => {
        $(
            $map.insert(stringify!($func_name).to_string(), SunValue::Function($func_name));
        )+
    };
}

#[macro_export]
macro_rules! include_const {
    ( $map:expr, $( $const_name:ident ),+ ) => {
        $(
            $map.insert(stringify!($const_name).to_string().to_ascii_lowercase(), $const_name);
        )+
    };
}

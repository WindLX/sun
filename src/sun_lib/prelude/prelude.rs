use crate::sun_lib::value::{
    sun_boolean::SunBoolean,
    sun_function::Function,
    sun_function::SunFunction,
    sun_nil::SunNil,
    sun_number::SunNumber,
    sun_object::{IsSunObject, SunObject, SunValue, _type},
    sun_pointer::SunPointer,
    sun_table::SunTable,
};
use crate::utils::log::log_output;
use crate::{add_metas, add_prelude_methods};
use std::{collections::HashMap, process};

pub fn prelude(
    value_map: &mut HashMap<String, SunPointer>,
    meta_map: &mut HashMap<&'static str, SunObject>,
) {
    add_metas!(
        meta_map,
        ("nil", SunNil),
        ("bool", SunBoolean),
        ("number", SunNumber),
        ("table", SunTable),
        ("function", SunFunction)
    );
    add_prelude_methods!(value_map, print, exit);
    value_map.insert("type".to_string(), SunPointer::new(SunValue::from(_type())));
}

#[macro_export]
macro_rules! add_prelude_methods {
    ($map:expr, $($func:ident),+) => {
        $(
            let name = stringify!($func);
            $map.insert(name.to_string(), SunPointer::new(SunValue::from($func as Function)));
        )+
    };
}

fn print(args: Vec<SunPointer>) -> Vec<SunPointer> {
    let log = args
        .iter()
        .map(|arg| arg.get().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    log_output(log);
    Vec::new()
}

fn exit(_: Vec<SunPointer>) -> Vec<SunPointer> {
    process::exit(0)
}

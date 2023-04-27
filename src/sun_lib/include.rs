use crate::vm::value::SunValue;
use std::collections::HashMap;

pub trait Include {
    fn include(global_map: &mut HashMap<String, SunValue>);
}

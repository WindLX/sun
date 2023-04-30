use crate::sun_lib::{sun_value::SunValue, Includable, Preludable, SunMod};
use crate::vm::machine::VirtualMachine;
use crate::{consts_none, funcs, funcs_none};
use colorized::*;
use std::collections::HashMap;

pub struct IO;

fn print(state: &mut VirtualMachine) -> u8 {
    let mut res = String::new();
    let num = state.len();
    if num == 0 {
        res.push_str(SunValue::Nil.to_string().as_str());
    }
    for _ in 0..num {
        let value = state.pop();
        res.push(' ');
        res.push_str(value.unwrap_or(SunValue::Nil).to_string().as_str());
    }
    println!("{}{}", "[o]".color(Colors::BrightYellowFg), res);
    num as u8
}

impl Includable for IO {
    fn generate_mod(&self) -> SunMod {
        let mut docs = HashMap::new();
        docs.insert("print".to_string(), SunValue::from("print: \n      summary: 向标准输出流输出字符\n      parameter: type(any), number(any)\n      return: nil"));
        // let docs = docs!();
        let consts = consts_none!();
        SunMod::new("io".to_string(), docs, consts, funcs_none!())
    }
}

impl Preludable for IO {
    fn generate_preclude_mod(&self) -> HashMap<String, SunValue> {
        funcs!(print)
    }
}

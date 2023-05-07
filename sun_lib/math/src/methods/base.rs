use sun_core::{
    container::{Function, RustFunction, SunValue},
    utils::SunPointer,
};

pub fn abs() -> Function {
    let f = |args: Vec<SunPointer>| {
        let arg = args[0].get();
        let mut self_value = args[0].borrow_mut();
        let res = match arg {
            SunValue::Number(n) => {
                *self_value = SunValue::from(n.abs());
                vec![args[0].clone()]
            }
            _ => Vec::new(),
        };
        res
    };
    Function::from(f as RustFunction)
}

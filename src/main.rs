pub mod parser;
pub mod sun_lib;
pub mod tokenizer;
pub mod utils;
pub mod vm;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut is_debug: bool = false;
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--debug") {
        is_debug = true;
    }
    match args.len() {
        1 => {
            todo!("交互式")
        }
        2 | 3 => match File::open(&args[1]) {
            Ok(f) => start(is_debug, f),
            Err(e) => {
                eprintln!("Invalid path or file: {e}");
                std::process::exit(0);
            }
        },
        _ => {}
    }
}

fn start<T: Read>(is_debug: bool, inner: T) {
    let input = BufReader::new(inner);
    let proto = parser::parser::ParseProto::new(input);
    vm::machine::VirtualMachine::new(is_debug).run(&proto);
}

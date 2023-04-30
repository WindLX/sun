use crate::parser::parser::ParseProto;
use crate::utils::config::Config;
use crate::utils::err::SunError;
use crate::vm::machine::VirtualMachine;
use colorized::*;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process;
use std::{env, io};

/*
    程序运行函数
*/
pub fn run() {
    let (args, config) = get_config();
    let mut vm = VirtualMachine::new(config.is_debug);
    // preclude(&mut vm);
    match args.len() {
        1 => loop {
            print!("{}", "[i] ".color(Colors::BrightGreenFg));
            io::stdout().flush().expect("failed to flush stdout");
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Ok(_) => {
                    let buf = buf.as_bytes();
                    vm.run(&ParseProto::new(
                        buf,
                        config.check_tokenizer,
                        config.check_parser,
                        config.check_command,
                    ));
                }
                Err(e) => {
                    eprintln!("{}", SunError::InputError(e.to_string()));
                    process::exit(0);
                }
            }
        },
        2 => match File::open(&args[1]) {
            Ok(f) => vm.run(&ParseProto::new(
                BufReader::new(f),
                config.check_tokenizer,
                config.check_parser,
                config.check_command,
            )),
            Err(_) => {
                eprintln!(
                    "{}",
                    SunError::InputError("failed to find target file".to_string())
                );
                process::exit(0);
            }
        },
        _ => {}
    }
}

/*
    处理运行时的配置
    return:
        (Vec<String>, Config): 剩余的命令参数和配置
*/
fn get_config() -> (Vec<String>, Config) {
    let mut config = Config::new();
    let mut args: Vec<String> = env::args().collect();
    if let Some(idx) = args.iter().position(|a| a == "--debug") {
        args.remove(idx);
        config.is_debug = true;
    }
    if let Some(idx) = args.iter().position(|a| a == "--ct") {
        args.remove(idx);
        config.check_tokenizer = true;
    }
    if let Some(idx) = args.iter().position(|a| a == "--cp") {
        args.remove(idx);
        config.check_parser = true;
    }
    if let Some(idx) = args.iter().position(|a| a == "--cc") {
        args.remove(idx);
        config.check_command = true;
    }
    (args, config)
}

/*
    预加载，在虚拟机运行前加载的sun代码
    para:
        vm: &VirtualMachine
*/
// fn preclude(vm: &mut VirtualMachine) {
//     match File::open("./preclude/preclude.sun") {
//         Ok(f) => vm.run(&ParseProto::new(BufReader::new(f), false, false, false)),
//         Err(_) => {
//             eprintln!(
//                 "{}",
//                 SunError::InputError("failed to find preclude file".to_string())
//             );
//             process::exit(0);
//         }
//     }
// }

use crate::{parser::parser::ParseProto, utils::config::Config, vm::machine::VirtualMachine};
use colorized::*;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process;
use std::{env, io};
use sun_core::utils::SunError;

/**
    sun 解释器程序运行的入口函数
*/
pub fn run() {
    let (args, config) = get_config();
    let mut vm = VirtualMachine::new(config.is_debug, config.check_stack, config.check_global);
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

/**
    `get_config` 处理运行时的配置，捕获命令行参数，生成配置，同时将配置参数从命令行参数容器中去除

    `return`: 去除配置后剩余的命令行参数
*/
fn get_config() -> (Vec<String>, Config) {
    let mut config = Config::new();
    let mut args: Vec<String> = env::args().collect();
    let flags = ["--debug", "--cs", "--cg", "--ct", "--cp", "--cc"];
    for flag in &flags {
        if let Some(idx) = args.iter().position(|a| a == flag) {
            args.remove(idx);
            match *flag {
                "--debug" => config.is_debug = true,
                "--cs" => config.check_stack = true,
                "--cg" => config.check_global = true,
                "--ct" => config.check_tokenizer = true,
                "--cp" => config.check_parser = true,
                "--cc" => config.check_command = true,
                _ => unreachable!(),
            }
        }
    }
    (args, config)
}

use colorized::*;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::process;

/// 输出错误信息
pub fn error_output<E>(error: E) -> !
where
    E: Error,
{
    eprint!("{}", "[e] ".color(Colors::RedFg));
    eprintln!("{} use `--debug` to get more information", error);
    process::exit(0);
}

/// 输出警告信息
pub fn warn_output<W>(warn: W)
where
    W: Display + Debug,
{
    eprint!("{}", "[w] ".color(Colors::YellowFg));
    eprintln!("{}", warn);
}

/// 输出日志信息
pub fn log_output<L>(log: L)
where
    L: Display + Debug,
{
    eprint!("{}", "[o] ".color(Colors::BrightYellowFg));
    eprintln!("{}", log);
}

/// 输出调试信息
pub fn debug_output<D>(debug: D, is_pretty: bool)
where
    D: Debug,
{
    eprint!("{}", "[d] ".color(Colors::BrightBlueFg));
    if is_pretty {
        eprintln!();
        eprintln!("{:#?}", debug);
    } else {
        eprintln!("{:?}", debug);
    }
}

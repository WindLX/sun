use colorized::*;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::process;

pub fn error_output<E>(error: E) -> !
where
    E: Error,
{
    eprint!("{}", "[e] ".color(Colors::RedFg));
    eprintln!("{} use `--debug` to get more information", error);
    process::exit(0);
}

pub fn warn_output<W>(warn: W)
where
    W: Display + Debug,
{
    eprint!("{}", "[w] ".color(Colors::YellowFg));
    eprintln!("{}", warn);
}

pub fn log_output<L>(log: L)
where
    L: Display + Debug,
{
    eprint!("{}", "[o] ".color(Colors::BrightYellowFg));
    eprintln!("{}", log);
}

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

use crate::tokenizer::{token::Token, tokenizer::Tokenizer};
use crate::utils::err::SunError;
use crate::vm::command::Command;
use crate::vm::value::SunValue;
use std::io::Read;

#[derive(Debug)]
pub struct ParseProto<T: Read> {
    pub commands: Vec<Command>,
    tokenizer: Tokenizer<T>,
}

impl<T: Read> ParseProto<T> {
    pub fn new(input: T) -> Self {
        let mut proto = ParseProto {
            commands: Vec::new(),
            tokenizer: Tokenizer::new(input),
        };
        proto.load();
        proto
    }

    fn load(&mut self) {
        loop {
            match self.tokenizer.next() {
                Some(Token::Name(name)) => match self.tokenizer.peek().unwrap() {
                    &Token::ParL => {
                        self.call_function(name);
                    }
                    &Token::Assign => {
                        self.assign_var(name);
                    }
                    _ => break,
                },
                Some(Token::Eos) | None => break,
                other => {
                    eprintln!(
                        "{}",
                        SunError::InvalidSymbolError(
                            format!("unexpected token {:?}", other),
                            self.tokenizer.line()
                        )
                    )
                }
            }
        }
    }

    fn assign_var(&mut self, var_name: String) {
        self.tokenizer.next();
        loop {
            match self.tokenizer.next() {
                Some(Token::True) => {
                    self.commands
                        .push(Command::SetValue(var_name, SunValue::Boolean(true)));
                    break;
                }
                Some(Token::False) => {
                    self.commands
                        .push(Command::SetValue(var_name, SunValue::Boolean(false)));
                    break;
                }
                Some(Token::Number(n)) => {
                    self.commands
                        .push(Command::SetValue(var_name, SunValue::Number(n)));
                    break;
                }
                Some(Token::Name(source)) => match self.tokenizer.peek() {
                    Some(&Token::ParL) => {
                        self.call_function(source);
                        self.commands
                            .push(Command::SetGlobalValue(var_name.clone()));
                        break;
                    }
                    _ => {
                        self.commands
                            .push(Command::CopyValue(source, var_name.clone()));
                        break;
                    }
                },
                None => break,
                other => panic!("TODO: invalid assign {other:?}"),
            }
        }
    }

    fn call_function(&mut self, func_name: String) {
        self.tokenizer.next();
        loop {
            match self.tokenizer.next().unwrap() {
                Token::True => self
                    .commands
                    .push(Command::AddValue(SunValue::Boolean(true))),
                Token::False => self
                    .commands
                    .push(Command::AddValue(SunValue::Boolean(false))),
                Token::Number(n) => self.commands.push(Command::AddValue(SunValue::Number(n))),
                Token::Name(n) => match self.tokenizer.peek() {
                    Some(&Token::ParL) => {
                        self.call_function(n);
                        break;
                    }
                    _ => {
                        self.commands.push(Command::LoadValue(n));
                        break;
                    }
                },
                Token::Nil => self.commands.push(Command::AddValue(SunValue::Nil)),
                Token::Comma => continue,
                Token::ParR => break,
                other => panic!("TODO: invalid call function {other:?}"),
            }
        }
        self.commands.push(Command::LoadFunc(func_name));
        self.commands.push(Command::Call);
    }
}

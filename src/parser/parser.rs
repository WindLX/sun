use crate::parser::expr::{trans, Expr};
use crate::tokenizer::{token::Token, tokenizer::Tokenizer};
use crate::utils::err::SunError;
use crate::vm::command::Command;
use crate::vm::value::SunValue;
use std::io::Read;
use std::process;

#[derive(Debug)]
pub struct ParseProto<T: Read> {
    pub commands: Vec<Command>,
    tokenizer: Tokenizer<T>,
    check: bool,
}

impl<T: Read> ParseProto<T> {
    pub fn new(input: T, check_tokenizer: bool, check_parser: bool) -> Self {
        let mut proto = ParseProto {
            commands: Vec::new(),
            tokenizer: Tokenizer::new(input, check_tokenizer),
            check: check_parser,
        };
        proto.load();
        proto
    }

    fn load(&mut self) {
        loop {
            let ast = self.parse_expr();
            if self.check {
                println!("{ast:#?}");
            }
            self.commands.append(&mut trans(ast));
            if let &Token::Eos = self.tokenizer.peek() {
                break;
            }
        }
    }

    fn parse_expr(&mut self) -> Box<Expr> {
        self.parse_0()
    }

    // add sub and or xor
    fn parse_0(&mut self) -> Box<Expr> {
        let mut left = self.parse_1();
        loop {
            match self.tokenizer.peek() {
                &Token::Add => {
                    self.tokenizer.next();
                    let right = self.parse_1();
                    left = Box::new(Expr::Add(left, right));
                }
                &Token::Sub => {
                    self.tokenizer.next();
                    let right = self.parse_1();
                    left = Box::new(Expr::Sub(left, right));
                }
                &Token::And => {
                    self.tokenizer.next();
                    let right = self.parse_1();
                    left = Box::new(Expr::And(left, right));
                }
                &Token::Or => {
                    self.tokenizer.next();
                    let right = self.parse_1();
                    left = Box::new(Expr::Or(left, right));
                }
                &Token::Xor => {
                    self.tokenizer.next();
                    let right = self.parse_1();
                    left = Box::new(Expr::Xor(left, right));
                }
                _ => break,
            }
        }
        left
    }

    // mul div mod
    fn parse_1(&mut self) -> Box<Expr> {
        let mut left = self.parse_2();
        loop {
            match self.tokenizer.peek() {
                &Token::Mul => {
                    self.tokenizer.next();
                    let right = self.parse_2();
                    left = Box::new(Expr::Mul(left, right));
                }
                &Token::Div => {
                    self.tokenizer.next();
                    let right = self.parse_2();
                    left = Box::new(Expr::Div(left, right));
                }
                &Token::Mod => {
                    self.tokenizer.next();
                    let right = self.parse_2();
                    left = Box::new(Expr::Mod(left, right));
                }
                _ => break,
            }
        }
        left
    }

    // pow
    fn parse_2(&mut self) -> Box<Expr> {
        let mut left = self.parse_3();
        loop {
            match self.tokenizer.peek() {
                &Token::Pow => {
                    self.tokenizer.next();
                    let right = self.parse_3();
                    left = Box::new(Expr::Pow(left, right));
                }
                _ => break,
            }
        }
        left
    }

    // neg not
    fn parse_3(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Sub => {
                self.tokenizer.next();
                Box::new(Expr::Neg(self.parse_func()))
            }
            &Token::Not => {
                self.tokenizer.next();
                Box::new(Expr::Not(self.parse_func()))
            }
            _ => self.parse_func(),
        }
    }

    fn parse_func(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Name(ref name) => {
                let mut args = Vec::new();
                let name = name.clone();
                self.tokenizer.next();
                match self.tokenizer.peek() {
                    &Token::ParL => {
                        self.tokenizer.next();
                        if self.tokenizer.peek() != &Token::ParR {
                            args.push(self.parse_expr());
                            while self.tokenizer.peek() == &Token::Comma {
                                self.tokenizer.next();
                                args.push(self.parse_expr());
                            }
                        }
                        self.expect(Token::ParR);
                        Box::new(Expr::Call(name, args))
                    }
                    &Token::Assign => {
                        self.tokenizer.next();
                        Box::new(Expr::Assign(name, self.parse_expr()))
                    }
                    _ => Box::new(Expr::Variable(name)),
                }
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Box<Expr> {
        match self.tokenizer.next() {
            Some(Token::Number(value)) => Box::new(Expr::Constant(SunValue::Number(value))),
            Some(Token::ParL) => {
                let expr = self.parse_expr();
                self.expect(Token::ParR);
                expr
            }
            Some(other) => {
                eprintln!(
                    "{}",
                    SunError::SymbolError(
                        format!("unexpected token `{other:?}`",),
                        self.tokenizer.line()
                    )
                );
                process::exit(0);
            }
            None => {
                eprintln!(
                    "{}",
                    SunError::SymbolError(format!("incomplete statement",), self.tokenizer.line())
                );
                process::exit(0);
            }
        }
    }

    fn expect(&mut self, token: Token) -> Token {
        match self.tokenizer.peek() {
            t if t == &token => self.tokenizer.next().unwrap(),
            other => {
                eprintln!(
                    "{}",
                    SunError::SymbolError(
                        format!("expected `{token:?}`, but got `{:?}`", other),
                        self.tokenizer.line()
                    )
                );
                process::exit(0);
            }
        };
        token
    }
}

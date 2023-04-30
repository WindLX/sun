use crate::parser::expr::{trans, Expr};
use crate::sun_lib::sun_value::SunValue;
use crate::tokenizer::{token::Token, tokenizer::Tokenizer};
use crate::utils::err::SunError;
use crate::vm::command::Command;
use std::io::Read;
use std::process;

#[derive(Debug)]
pub struct ParseProto<T: Read> {
    pub commands: Vec<Command>,
    tokenizer: Tokenizer<T>,
    check: bool,
    check_command: bool,
}

impl<T: Read> ParseProto<T> {
    pub fn new(input: T, check_tokenizer: bool, check_parser: bool, check_command: bool) -> Self {
        let mut proto = ParseProto {
            commands: Vec::new(),
            tokenizer: Tokenizer::new(input, check_tokenizer),
            check: check_parser,
            check_command: check_command,
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
            self.commands.append(&mut trans(ast, self.check_command));
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

    // neg not conj
    fn parse_3(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Sub => {
                self.tokenizer.next();
                Box::new(Expr::Neg(self.parse_4()))
            }
            &Token::Not => {
                self.tokenizer.next();
                Box::new(Expr::Not(self.parse_4()))
            }
            &Token::Mul => {
                self.tokenizer.next();
                Box::new(Expr::Conj(self.parse_4()))
            }
            _ => self.parse_4(),
        }
    }

    // fac
    fn parse_4(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Fac => {
                self.tokenizer.next();
                Box::new(Expr::Fac(self.parse_5()))
            }
            _ => self.parse_5(),
        }
    }

    // function call and assign
    fn parse_5(&mut self) -> Box<Expr> {
        let name = self.parse_6();
        match self.tokenizer.peek() {
            &Token::ParL => {
                let mut args = Vec::new();
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
                match *name {
                    Expr::Variable(n) => Box::new(Expr::Assign(n, self.parse_expr())),
                    d @ (Expr::Dot(_, _) | Expr::Index(_, _)) => {
                        Box::new(Expr::TableAssign(Box::new(d), self.parse_expr()))
                    }
                    _ => {
                        eprintln!(
                            "{}",
                            SunError::AssignError(
                                format!("invalid assigment statement"),
                                self.tokenizer.line() as u64
                            )
                        );
                        process::exit(0);
                    }
                }
            }
            _ => name,
        }
    }

    // dot index
    fn parse_6(&mut self) -> Box<Expr> {
        let mut left = self.parse_7();
        loop {
            match self.tokenizer.peek() {
                &Token::Dot => {
                    self.tokenizer.next();
                    let right = self.parse_string();
                    left = Box::new(Expr::Dot(left, right));
                }
                &Token::SquL => {
                    self.tokenizer.next();
                    let right;
                    if self.tokenizer.peek() != &Token::SquR {
                        right = self.parse_expr();
                    } else {
                        right = Box::new(Expr::Constant(SunValue::from(0.0)));
                    }
                    self.expect(Token::SquR);
                    left = Box::new(Expr::Index(left, right));
                }
                _ => break,
            }
        }
        left
    }

    // name
    fn parse_7(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Name(ref name) => {
                let name = name.clone();
                self.tokenizer.next();
                Box::new(Expr::Variable(name))
            }
            _ => self.parse_primary(),
        }
    }

    // string
    fn parse_string(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Name(ref name) => {
                let name = name.clone();
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::from(name)))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Number(ref value) => {
                let value = value.clone();
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::from(value)))
            }
            &Token::String(ref value) => {
                let value = value.clone();
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::from(value)))
            }
            &Token::True => {
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::from(true)))
            }
            &Token::False => {
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::from(false)))
            }
            &Token::Nil => {
                self.tokenizer.next();
                Box::new(Expr::Constant(SunValue::Nil))
            }
            &Token::ParL => {
                self.tokenizer.next();
                let expr = self.parse_expr();
                self.expect(Token::ParR);
                expr
            }
            &Token::Eos => {
                eprintln!(
                    "{}",
                    SunError::SymbolError(format!("incomplete statement",), self.tokenizer.line())
                );
                process::exit(0);
            }
            other => {
                eprintln!(
                    "{}",
                    SunError::SymbolError(
                        format!("unexpected token `{:?}`", other.clone()),
                        self.tokenizer.line()
                    )
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

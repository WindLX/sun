use crate::parser::expr::{trans, Expr};
use crate::sun_lib::value::sun_object::SunValue;
use crate::tokenizer::{token::Token, tokenizer::Tokenizer};
use crate::utils::{
    err::SunError,
    log::{debug_output, error_output},
};
use crate::vm::command::Command;
use std::io::Read;

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
            check_command,
        };
        proto.load();
        proto
    }

    fn load(&mut self) {
        loop {
            let ast = self.parse_expr();
            if self.check {
                debug_output(&ast, true);
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
                    left = Box::new(Expr::Rem(left, right));
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
                Box::new(Expr::Neg(self.parse_3()))
            }
            &Token::Not => {
                self.tokenizer.next();
                Box::new(Expr::Not(self.parse_3()))
            }
            &Token::Mul => {
                self.tokenizer.next();
                Box::new(Expr::Conj(self.parse_3()))
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
                match *name.clone() {
                    Expr::Dot(_, _) => Box::new(Expr::DotCall(name, args)),
                    _ => Box::new(Expr::Call(name, args)),
                }
            }
            &Token::Assign => {
                self.tokenizer.next();
                match *name {
                    Expr::Variable(n) => Box::new(Expr::Assign(n, self.parse_expr())),
                    ta @ (Expr::Index(_, _) | Expr::Dot(_, _)) => {
                        Box::new(Expr::TableAssign(Box::new(ta), self.parse_expr()))
                    }
                    _ => {
                        let e = SunError::AssignError(format!(
                            "invalid assigment statement at line {}",
                            self.tokenizer.line()
                        ));
                        error_output(e)
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
                    match self.tokenizer.peek() {
                        &Token::Name(ref name) => {
                            let name = name.clone();
                            self.tokenizer.next();
                            let right = Box::new(Expr::Constant(SunValue::from(name)));
                            left = Box::new(Expr::Dot(left, right));
                        }
                        _ => {
                            let e = SunError::AttributeError(format!(
                                "invalid get-attribute statement at line {}",
                                self.tokenizer.line()
                            ));
                            error_output(e)
                        }
                    }
                }
                &Token::SquL => {
                    self.tokenizer.next();
                    match self.tokenizer.peek() {
                        &Token::Number(idx) => {
                            self.tokenizer.next();
                            let right = Box::new(Expr::Constant(SunValue::from(idx)));
                            left = Box::new(Expr::Index(left, right));
                        }
                        &Token::String(ref key) => {
                            let key = key.clone();
                            self.tokenizer.next();
                            let right = Box::new(Expr::Constant(SunValue::from(key)));
                            left = Box::new(Expr::Index(left, right));
                        }
                        _ => {
                            let e = SunError::IndexError(format!(
                                "invalid index statement at line {}",
                                self.tokenizer.line()
                            ));
                            error_output(e)
                        }
                    }
                    self.expect(Token::SquR);
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
            &Token::CurL => {
                let mut args = Vec::new();
                self.tokenizer.next();
                if self.tokenizer.peek() != &Token::CurR {
                    args.push(self.parse_pair());
                    while self.tokenizer.peek() == &Token::Comma {
                        self.tokenizer.next();
                        args.push(self.parse_pair());
                    }
                }
                self.expect(Token::CurR);
                Box::new(Expr::TableCreate(args))
                // match *name.clone() {
                //     Expr::Assign(_, _) | Expr::TableAssign(_, _) => {
                //         Box::new(Expr::TableCreate(args))
                //     }
                //     _ => {
                //         let e = SunError::AssignError(format!(
                //             "invalid table-create statement at line {}",
                //             self.tokenizer.line()
                //         ));
                //         error_output(e)
                //     }
                // }
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_pair(&mut self) -> Box<Expr> {
        let left = self.parse_7();
        match self.tokenizer.peek() {
            &Token::Colon => {
                self.tokenizer.next();
                let right = self.parse_7();
                match *left {
                    Expr::Constant(key) => match key {
                        SunValue::String(key) => Box::new(Expr::PairCreate(key, right)),
                        other => {
                            let e = SunError::KeyError(format!(
                                "`{other}` is not a valid key at line {}",
                                self.tokenizer.line()
                            ));
                            error_output(e)
                        }
                    },
                    _ => {
                        let e = SunError::KeyError(format!(
                            "expression is not a valid key at line {}",
                            self.tokenizer.line()
                        ));
                        error_output(e)
                    }
                }
            }
            _ => left,
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
                let e = SunError::SymbolError(format!(
                    "incomplete statement at line {}",
                    self.tokenizer.line()
                ));
                error_output(e)
            }
            other => {
                let e = SunError::SymbolError(format!(
                    "unexpected token `{:?}` at line {}",
                    other.clone(),
                    self.tokenizer.line()
                ));
                error_output(e)
            }
        }
    }

    fn expect(&mut self, token: Token) -> Token {
        match self.tokenizer.peek() {
            t if t == &token => self.tokenizer.next().unwrap(),
            other => {
                let e =
                    SunError::SymbolError(format!("expected `{token:?}`, but got `{:?}`", other));
                error_output(e);
            }
        };
        token
    }
}

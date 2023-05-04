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
        match self.tokenizer.peek() {
            &Token::DefClass => self.parse_defclass(),
            &Token::DefFunction => self.parse_deffunc(),
            _ => self.parse_control(),
        }
    }

    fn parse_control(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::If => self.parse_if(),
            &Token::Loop => self.parse_loop(),
            _ => self.parse_0(),
        }
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
                    args.push(self.parse_0());
                    while self.tokenizer.peek() == &Token::Comma {
                        self.tokenizer.next();
                        args.push(self.parse_0());
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
                    Expr::Variable(n) => Box::new(Expr::Assign(n, self.parse_0())),
                    ta @ (Expr::Index(_, _) | Expr::Dot(_, _)) => {
                        Box::new(Expr::TableAssign(Box::new(ta), self.parse_0()))
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
                                "invalid get attribute statement because of invalid token at line {}",
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
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_pair(&mut self) -> Box<Expr> {
        let left = self.parse_0();
        match self.tokenizer.peek() {
            &Token::Colon => {
                self.tokenizer.next();
                let right = self.parse_0();
                match *left {
                    Expr::Constant(key) => match key {
                        key @ SunValue::String(_) => {
                            Box::new(Expr::PairCreate((&key).into(), right))
                        }
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

    // def class
    fn parse_defclass(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::DefClass => {
                self.tokenizer.next();
                todo!("def class")
            }
            _ => self.parse_0(),
        }
    }

    // def function
    fn parse_deffunc(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::DefClass => {
                self.tokenizer.next();
                todo!("def function")
            }
            _ => self.parse_0(),
        }
    }

    // condition
    fn parse_cond(&mut self) -> Box<Expr> {
        let mut left = self.parse_0();
        self.unexpect_assign(&mut left);
        match self.tokenizer.peek() {
            &Token::Eq => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::Eq(left, right))
            }
            &Token::NotEq => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::NotEq(left, right))
            }
            &Token::Le => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::Le(left, right))
            }
            &Token::Ge => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::Ge(left, right))
            }
            &Token::Less => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::Less(left, right))
            }
            &Token::Greater => {
                self.tokenizer.next();
                let mut right = self.parse_0();
                self.unexpect_assign(&mut right);
                Box::new(Expr::Greater(left, right))
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

    // if
    fn parse_if(&mut self) -> Box<Expr> {
        self.expect(Token::If);
        let cond = self.parse_cond();
        self.expect(Token::Colon);
        let then_expr = self.parse_control();
        let else_expr = if let &Token::Else = self.tokenizer.peek() {
            self.tokenizer.next();
            Some(self.parse_expr())
        } else {
            None
        };
        self.expect(Token::Semi);
        Box::new(Expr::If(cond, then_expr, else_expr))
    }

    // loop
    fn parse_loop(&mut self) -> Box<Expr> {
        self.expect(Token::Loop);
        let cond = self.parse_cond();
        self.expect(Token::Colon);
        let body = self.parse_expr();
        self.expect(Token::Semi);
        Box::new(Expr::Loop(cond, body))
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
                let expr = self.parse_0();
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

    fn unexpect_assign(&mut self, expr: &mut Box<Expr>) {
        if let Expr::Assign(_, _) | Expr::TableAssign(_, _) = *(*expr) {
            let e = SunError::SymbolError(format!(
                "assign statement can't be condition at line {}",
                self.tokenizer.line()
            ));
            error_output(e)
        }
    }
}

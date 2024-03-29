use crate::{
    parser::expr::{trans, Expr},
    tokenizer::{token::Token, tokenizer::Tokenizer},
    vm::command::Command,
};
use std::io::Read;
use sun_core::{
    container::SunValue,
    utils::{
        err::SunError,
        log::{debug_output, error_output},
    },
};

/// 语法分析器的结构体
#[derive(Debug)]
pub struct ParseProto<T: Read> {
    /// 生成的指令序列
    pub commands: Vec<Command>,
    /// 词法分析器
    tokenizer: Tokenizer<T>,
    /// 检查语法树的标志
    check: bool,
    /// 检查生成命令的标志
    check_command: bool,
}

impl<T: Read> ParseProto<T> {
    /// 创建新的语法分析器
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

    /// 进行语法分析
    fn load(&mut self) {
        loop {
            let ast = self.parse_block();
            if self.check {
                debug_output(&ast, true);
            }
            self.commands.append(&mut trans(ast, self.check_command));
            match self.tokenizer.peek() {
                &Token::Eos => break,
                &Token::Semi => {
                    self.tokenizer.next();
                    continue;
                }
                _ => break,
            }
        }
    }

    /// 语句段：定义语段
    fn parse_chunk(&mut self) -> Vec<Box<Expr>> {
        let mut blocks = Vec::new();
        if matches!(self.tokenizer.peek(), Token::CurR) {
            self.tokenizer.next();
        } else {
            blocks.push(self.parse_block());
            loop {
                match self.tokenizer.peek() {
                    &Token::Semi => {
                        self.tokenizer.next();
                        blocks.push(self.parse_block());
                        continue;
                    }
                    &Token::CurR => {
                        self.tokenizer.next();
                        break;
                    }
                    other => {
                        let e = SunError::SymbolError(format!(
                            "unexpected token `{:?}` at line {}",
                            other.clone(),
                            self.tokenizer.line()
                        ));
                        error_output(e)
                    }
                };
            }
        }
        blocks
    }

    /// 语句块：流程控制语段或表达式
    fn parse_block(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Import => self.parse_import(),
            &Token::DefFunction => self.parse_def(),
            &Token::If | &Token::Loop => self.parse_control(),
            _ => self.parse_expr(),
        }
    }

    /// 表达式语句
    fn parse_expr(&mut self) -> Box<Expr> {
        self.parse_logic()
    }

    /// 流程控制语句
    fn parse_control(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::If => self.parse_if(),
            &Token::Loop => self.parse_loop(),
            _ => unreachable!("parse control"),
        }
    }

    /// 定义语句
    fn parse_def(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::DefFunction => self.parse_deffunc(),
            _ => unreachable!("parse def"),
        }
    }

    /// 导入模块语句
    fn parse_import(&mut self) -> Box<Expr> {
        match self.tokenizer.next() {
            Some(Token::Import) => match self.tokenizer.peek() {
                &Token::String(ref lib_name) => {
                    Box::new(Expr::Import(String::from_utf8_lossy(&lib_name).to_string()))
                }
                other => {
                    let e =
                        SunError::SymbolError(format!("expected lib name, but got `{:?}`", other));
                    error_output(e);
                }
            },
            _ => unreachable!("parse import"),
        }
    }

    /// and or xor
    fn parse_logic(&mut self) -> Box<Expr> {
        let mut left = self.parse_compare();
        loop {
            match self.tokenizer.peek() {
                &Token::And => {
                    self.tokenizer.next();
                    let right = self.parse_compare();
                    left = Box::new(Expr::And(left, right));
                }
                &Token::Or => {
                    self.tokenizer.next();
                    let right = self.parse_compare();
                    left = Box::new(Expr::Or(left, right));
                }
                &Token::Xor => {
                    self.tokenizer.next();
                    let right = self.parse_compare();
                    left = Box::new(Expr::Xor(left, right));
                }
                _ => break,
            }
        }
        left
    }

    /// compare
    fn parse_compare(&mut self) -> Box<Expr> {
        let left = self.parse_0();
        match self.tokenizer.peek() {
            &Token::Eq => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::Eq(left, right))
            }
            &Token::NotEq => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::NotEq(left, right))
            }
            &Token::Le => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::Le(left, right))
            }
            &Token::Ge => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::Ge(left, right))
            }
            &Token::Less => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::Less(left, right))
            }
            &Token::Greater => {
                self.tokenizer.next();
                let right = self.parse_0();
                Box::new(Expr::Greater(left, right))
            }
            _ => left,
        }
    }

    /// add sub and or xor
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
                _ => break,
            }
        }
        left
    }

    /// mul div mod
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

    /// neg not
    fn parse_2(&mut self) -> Box<Expr> {
        match self.tokenizer.peek() {
            &Token::Sub => {
                self.tokenizer.next();
                Box::new(Expr::Neg(self.parse_2()))
            }
            &Token::Not => {
                self.tokenizer.next();
                Box::new(Expr::Not(self.parse_2()))
            }
            _ => self.parse_3(),
        }
    }

    /// function call and assign
    fn parse_3(&mut self) -> Box<Expr> {
        let name = self.parse_4();
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

    /// dot index
    fn parse_4(&mut self) -> Box<Expr> {
        let mut left = self.parse_metacall();
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
                        &Token::Name(ref name) => {
                            let name = name.clone();
                            self.tokenizer.next();
                            let right = Box::new(Expr::Variable(name));
                            left = Box::new(Expr::Index(left, right));
                        }
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

    /// metacall
    fn parse_metacall(&mut self) -> Box<Expr> {
        let name = self.parse_5();
        match self.tokenizer.peek() {
            &Token::Colon => {
                self.tokenizer.next();
                let method = self.parse_5();
                match (*name, *method) {
                    (Expr::Variable(n), Expr::Variable(m)) => Box::new(Expr::MetaCall(n, m)),
                    _ => {
                        let e = SunError::CallError(format!(
                            "invalid meta call statement at line {}",
                            self.tokenizer.line()
                        ));
                        error_output(e)
                    }
                }
            }
            _ => name,
        }
    }

    /// name
    fn parse_5(&mut self) -> Box<Expr> {
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

    /// key-value pair
    fn parse_pair(&mut self) -> Box<Expr> {
        let left = self.parse_primary();
        match self.tokenizer.peek() {
            &Token::Colon => {
                self.tokenizer.next();
                let right = self.parse_expr();
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

    /// def function
    fn parse_deffunc(&mut self) -> Box<Expr> {
        self.parse_0()
    }

    /// if
    fn parse_if(&mut self) -> Box<Expr> {
        self.expect(Token::If);
        let mut cond = self.parse_logic_unassign();
        self.unexpect_assign(&mut cond);
        self.expect(Token::CurL);
        let thens = self.parse_chunk();
        let elses = if let &Token::Else = self.tokenizer.peek() {
            self.tokenizer.next();
            let elses = self.parse_chunk();
            Some(elses)
        } else {
            None
        };
        Box::new(Expr::If(cond, thens, elses))
    }

    /// loop
    fn parse_loop(&mut self) -> Box<Expr> {
        self.expect(Token::Loop);
        let mut cond = self.parse_logic_unassign();
        self.unexpect_assign(&mut cond);
        self.expect(Token::CurL);
        let bodys = self.parse_chunk();
        Box::new(Expr::Loop(cond, bodys))
    }

    /// 禁止包含赋值语句的 and or xor
    fn parse_logic_unassign(&mut self) -> Box<Expr> {
        let mut left = self.parse_compare_unassign();
        self.unexpect_assign(&mut left);
        loop {
            match self.tokenizer.peek() {
                &Token::And => {
                    self.tokenizer.next();
                    let mut right = self.parse_compare_unassign();
                    self.unexpect_assign(&mut right);
                    left = Box::new(Expr::And(left, right));
                }
                &Token::Or => {
                    self.tokenizer.next();
                    let mut right = self.parse_compare_unassign();
                    self.unexpect_assign(&mut right);
                    left = Box::new(Expr::Or(left, right));
                }
                &Token::Xor => {
                    self.tokenizer.next();
                    let mut right = self.parse_compare_unassign();
                    self.unexpect_assign(&mut right);
                    left = Box::new(Expr::Xor(left, right));
                }
                _ => break,
            }
        }
        left
    }

    /// 禁止包含赋值语句的 compare
    fn parse_compare_unassign(&mut self) -> Box<Expr> {
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
            _ => left,
        }
    }

    /// 原子语句
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

    /// 检查下一个 `Token` 是否为期望的 `Token`，否则打印错误
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

    /// 检查当前语句是否不为赋值语句
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

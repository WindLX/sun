use crate::vm::command::Command;
use crate::vm::value::SunValue;

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>), // 0
    Sub(Box<Expr>, Box<Expr>), // 0
    Mul(Box<Expr>, Box<Expr>), // 1
    Div(Box<Expr>, Box<Expr>), // 1
    Mod(Box<Expr>, Box<Expr>), // 1
    Pow(Box<Expr>, Box<Expr>), // 2
    Neg(Box<Expr>),            // 3
    And(Box<Expr>, Box<Expr>), // 0
    Or(Box<Expr>, Box<Expr>),  // 0
    Not(Box<Expr>),            // 3
    Xor(Box<Expr>, Box<Expr>), // 0
    Assign(String, Box<Expr>),
    Call(String, Vec<Box<Expr>>), // 4
    Constant(SunValue),
    Variable(String),
    None,
}

pub fn trans(ast: Box<Expr>) -> Vec<Command> {}

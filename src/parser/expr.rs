use crate::sun_lib::sun_value::SunValue;
use crate::vm::command::Command;

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),   // 0
    Sub(Box<Expr>, Box<Expr>),   // 0
    Mul(Box<Expr>, Box<Expr>),   // 1
    Div(Box<Expr>, Box<Expr>),   // 1
    Mod(Box<Expr>, Box<Expr>),   // 1
    Pow(Box<Expr>, Box<Expr>),   // 2
    Neg(Box<Expr>),              // 3
    Fac(Box<Expr>),              // 4
    Conj(Box<Expr>),             // 4
    And(Box<Expr>, Box<Expr>),   // 0
    Or(Box<Expr>, Box<Expr>),    // 0
    Not(Box<Expr>),              // 3
    Xor(Box<Expr>, Box<Expr>),   // 0
    Dot(Box<Expr>, Box<Expr>),   // 6
    Index(Box<Expr>, Box<Expr>), // 6
    Assign(String, Box<Expr>),
    TableAssign(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Box<Expr>>), // 5
    Constant(SunValue),
    Variable(String),
}

#[derive(Debug)]
pub enum Desc {
    Single(String),
    Double(String),
    Dot,
    Index,
    Assign(String),
    TableAssign,
    Call(u8),
    Constant(SunValue),
    Variable(String),
}

pub fn trans(ast: Box<Expr>, check: bool) -> Vec<Command> {
    let mut expr_stack: Vec<Desc> = Vec::new();
    traverse_expr(&mut expr_stack, &ast);
    let mut commands: Vec<Command> = Vec::new();
    for desc in expr_stack {
        match desc {
            Desc::Single(f) => {
                commands.push(Command::Call(1));
                commands.push(Command::LoadFunc(f));
            }
            Desc::Double(f) => {
                commands.push(Command::Call(2));
                commands.push(Command::LoadFunc(f));
            }
            Desc::Call(n) => {
                commands.push(Command::Call(n));
            }
            Desc::Dot => commands.push(Command::LoadTableValueByKey),
            Desc::Index => commands.push(Command::LoadTableValueByIndex),
            Desc::Variable(v) => commands.push(Command::LoadValue(v)),
            Desc::Constant(c) => commands.push(Command::AddValue(c)),
            Desc::Assign(n) => commands.push(Command::SetGlobalValue(n)),
            Desc::TableAssign => commands.push(Command::SetTableValue),
        }
    }
    commands.reverse();
    if check == true {
        println!("{:?}", commands);
    }
    commands
}

fn traverse_expr(expr_stack: &mut Vec<Desc>, expr: &Expr) {
    match expr {
        Expr::Add(left, right) => {
            expr_stack.push(Desc::Double("add".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Sub(left, right) => {
            expr_stack.push(Desc::Double("sub".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Mul(left, right) => {
            expr_stack.push(Desc::Double("mul".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Div(left, right) => {
            expr_stack.push(Desc::Double("div".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Mod(left, right) => {
            expr_stack.push(Desc::Double("mod".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Pow(left, right) => {
            expr_stack.push(Desc::Double("pow".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::And(left, right) => {
            expr_stack.push(Desc::Double("and".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Or(left, right) => {
            expr_stack.push(Desc::Double("or".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Xor(left, right) => {
            expr_stack.push(Desc::Double("xor".to_string()));
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Dot(left, right) => {
            expr_stack.push(Desc::Dot);
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Index(left, right) => {
            expr_stack.push(Desc::Index);
            traverse_expr(expr_stack, left);
            traverse_expr(expr_stack, right);
        }
        Expr::Neg(left) => {
            expr_stack.push(Desc::Single("neg".to_string()));
            traverse_expr(expr_stack, left);
        }
        Expr::Not(left) => {
            expr_stack.push(Desc::Single("not".to_string()));
            traverse_expr(expr_stack, left);
        }
        Expr::Fac(left) => {
            expr_stack.push(Desc::Single("fac".to_string()));
            traverse_expr(expr_stack, left);
        }
        Expr::Conj(left) => {
            expr_stack.push(Desc::Single("conj".to_string()));
            traverse_expr(expr_stack, left);
        }
        Expr::Constant(value) => {
            expr_stack.push(Desc::Constant(value.clone()));
        }
        Expr::Variable(name) => expr_stack.push(Desc::Variable(name.to_string())),
        Expr::Assign(name, expr) => {
            expr_stack.push(Desc::Assign(name.to_string()));
            traverse_expr(expr_stack, expr);
        }
        Expr::TableAssign(name, value) => {
            expr_stack.push(Desc::TableAssign);
            traverse_expr(expr_stack, name);
            traverse_expr(expr_stack, value);
        }
        Expr::Call(name, args) => {
            expr_stack.push(Desc::Call(args.len() as u8));
            traverse_expr(expr_stack, name);
            for arg in args {
                traverse_expr(expr_stack, arg);
            }
        }
    }
}

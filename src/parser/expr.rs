use crate::sun_lib::value::sun_object::SunValue;
use crate::utils::log::debug_output;
use crate::vm::command::Command;

#[derive(Debug, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),   // 0
    Sub(Box<Expr>, Box<Expr>),   // 0
    Mul(Box<Expr>, Box<Expr>),   // 1
    Div(Box<Expr>, Box<Expr>),   // 1
    Rem(Box<Expr>, Box<Expr>),   // 1
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
    TableCreate(Vec<Box<Expr>>),
    PairCreate(String, Box<Expr>),
    Call(Box<Expr>, Vec<Box<Expr>>),    // 5
    DotCall(Box<Expr>, Vec<Box<Expr>>), // 5
    Constant(SunValue),
    Variable(String),
    // condition
    Eq(Box<Expr>, Box<Expr>),
    NotEq(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    // if loop
    If(Box<Expr>, Vec<Box<Expr>>, Option<Vec<Box<Expr>>>),
    Loop(Box<Expr>, Vec<Box<Expr>>),
}

#[derive(Debug)]
pub enum Desc {
    Single(String),
    Double(String),
    Dot,
    Index,
    Assign(String),
    TableAssign,
    TableCreate(usize),
    PairCreate(String),
    Call(usize),
    Constant(SunValue),
    Variable(String),
    If,
    IfTrueEnd,
    IfFalse,
    IfEnd,
    Loop,
    LoopStart,
    LoopEnd,
}

pub fn trans(ast: Box<Expr>, check: bool) -> Vec<Command> {
    let mut expr_stack: Vec<Desc> = Vec::new();
    traverse_expr(&mut expr_stack, &ast);
    // println!("{:?}", expr_stack);
    let mut commands: Vec<Command> = Vec::new();
    for (position, desc) in expr_stack.iter().enumerate() {
        match desc {
            Desc::Single(f) => {
                commands.push(Command::LoadMethod(f.to_owned()));
                commands.push(Command::Call(1));
            }
            Desc::Double(f) => {
                commands.push(Command::LoadMethod(f.to_owned()));
                commands.push(Command::Call(2));
            }
            Desc::Call(n) => {
                commands.push(Command::Call(*n));
            }
            Desc::Dot => commands.push(Command::LoadMethod("dot".to_string())),
            Desc::Index => {
                commands.push(Command::LoadMethod("index".to_string()));
                commands.push(Command::Call(2));
            }
            Desc::Variable(v) => commands.push(Command::LoadValue(v.to_owned())),
            Desc::Constant(c) => commands.push(Command::LoadConst(c.to_owned())),
            Desc::Assign(n) => commands.push(Command::StoreGlobal(n.to_owned())),
            Desc::TableAssign => commands.push(Command::SetTable),
            Desc::TableCreate(n) => commands.push(Command::CreateTable(n.to_owned())),
            Desc::PairCreate(k) => commands.push(Command::SetPair(k.to_owned())),
            Desc::If => {
                let if_false_pos = count_desc_distance(&expr_stack, position, &Desc::IfFalse);
                let if_end_pos = count_desc_distance(&expr_stack, position, &Desc::IfEnd).unwrap();
                match if_false_pos {
                    Some(pos) => commands.push(Command::TestJump(pos - 1)),
                    None => commands.push(Command::TestJump(if_end_pos - 1)),
                }
            }
            Desc::IfTrueEnd => {
                let if_end_pos = count_desc_distance(&expr_stack, position, &Desc::IfEnd).unwrap();
                commands.push(Command::Jump(if_end_pos - 1));
            }
            Desc::IfFalse => continue,
            Desc::IfEnd => continue,
            Desc::Loop => continue,
            Desc::LoopStart => {
                let loop_end_pos =
                    count_desc_distance(&expr_stack, position, &Desc::LoopEnd).unwrap();
                commands.push(Command::TestJump(loop_end_pos + 1));
            }
            Desc::LoopEnd => {
                let loop_pos =
                    reverse_count_desc_distance(&expr_stack, position, &Desc::Loop).unwrap();
                commands.push(Command::Back(loop_pos));
            }
        }
    }
    if check == true {
        debug_output(&commands, false);
    }
    commands
}

fn traverse_expr(expr_stack: &mut Vec<Desc>, expr: &Expr) {
    match expr {
        Expr::Add(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("add".to_string()));
        }
        Expr::Sub(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("sub".to_string()));
        }
        Expr::Mul(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("mul".to_string()));
        }
        Expr::Div(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("div".to_string()));
        }
        Expr::Rem(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("rem".to_string()));
        }
        Expr::Pow(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("pow".to_string()));
        }
        Expr::And(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("and".to_string()));
        }
        Expr::Or(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("or".to_string()));
        }
        Expr::Xor(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("xor".to_string()));
        }
        Expr::Dot(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Dot);
        }
        Expr::Index(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Index);
        }
        Expr::Neg(left) => {
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Single("neg".to_string()));
        }
        Expr::Not(left) => {
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Single("not".to_string()));
        }
        Expr::Fac(left) => {
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Single("fac".to_string()));
        }
        Expr::Conj(left) => {
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Single("conj".to_string()));
        }
        Expr::Eq(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("eq".to_string()));
        }
        Expr::NotEq(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("noteq".to_string()));
        }
        Expr::Le(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("le".to_string()));
        }
        Expr::Ge(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("ge".to_string()));
        }
        Expr::Less(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("less".to_string()));
        }
        Expr::Greater(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::Double("greater".to_string()));
        }
        Expr::Constant(value) => {
            expr_stack.push(Desc::Constant(value.clone()));
        }
        Expr::Variable(name) => expr_stack.push(Desc::Variable(name.to_owned())),
        Expr::Assign(name, expr) => {
            traverse_expr(expr_stack, expr);
            expr_stack.push(Desc::Assign(name.to_owned()));
        }
        Expr::TableAssign(left, right) => {
            traverse_expr(expr_stack, right);
            traverse_expr(expr_stack, left);
            expr_stack.push(Desc::TableAssign);
        }
        Expr::Call(name, args) => {
            for arg in args.iter().rev() {
                traverse_expr(expr_stack, arg);
            }
            traverse_expr(expr_stack, name);
            expr_stack.push(Desc::Call(args.len()));
        }
        Expr::DotCall(name, args) => {
            for arg in args.iter().rev() {
                traverse_expr(expr_stack, arg);
            }
            traverse_expr(expr_stack, name);
            expr_stack.push(Desc::Call(args.len() + 1));
        }
        Expr::TableCreate(values) => {
            for value in values.iter().rev() {
                traverse_expr(expr_stack, value);
            }
            expr_stack.push(Desc::TableCreate(values.len()));
        }
        Expr::PairCreate(key, value) => {
            traverse_expr(expr_stack, value);
            expr_stack.push(Desc::PairCreate(key.to_string()));
        }
        Expr::If(cond, thens, elses) => {
            traverse_expr(expr_stack, cond);
            expr_stack.push(Desc::If);
            for then in thens {
                traverse_expr(expr_stack, then);
            }
            expr_stack.push(Desc::IfTrueEnd);
            if let Some(elses) = elses {
                expr_stack.push(Desc::IfFalse);
                for else_ in elses {
                    traverse_expr(expr_stack, else_);
                }
            }
            expr_stack.push(Desc::IfEnd);
        }
        Expr::Loop(cond, bodys) => {
            expr_stack.push(Desc::Loop);
            traverse_expr(expr_stack, cond);
            expr_stack.push(Desc::LoopStart);
            for body in bodys {
                traverse_expr(expr_stack, body);
            }
            expr_stack.push(Desc::LoopEnd);
        }
    }
}

fn count_desc_distance(
    expr_stack: &Vec<Desc>,
    position: usize,
    target_desc: &Desc,
) -> Option<usize> {
    let mut count = 0;
    let mut is_found = false;
    let p = expr_stack[position..].iter().position(|desc| match desc {
        t if t == target_desc => {
            is_found = true;
            true
        }
        Desc::Single(_) | Desc::Double(_) | Desc::Index => {
            if is_found == false {
                count += 1
            }
            false
        }
        _ => false,
    });
    if let Some(p) = p {
        Some(p + count)
    } else {
        None
    }
}

fn reverse_count_desc_distance(
    expr_stack: &[Desc],
    position: usize,
    target_desc: &Desc,
) -> Option<usize> {
    let mut count = 0;
    let mut is_found = false;
    let p = expr_stack[..=position].iter().rposition(|desc| match desc {
        t if t == target_desc => {
            is_found = true;
            true
        }
        Desc::Single(_) | Desc::Double(_) | Desc::Index => {
            if is_found == false {
                count += 1
            }
            false
        }
        _ => false,
    });
    if let Some(mut p) = p {
        p = position - p;
        Some(p + count)
    } else {
        None
    }
}

impl PartialEq for Desc {
    fn eq(&self, other: &Self) -> bool {
        use Desc::*;
        match (self, other) {
            (IfTrueEnd, IfTrueEnd) => true,
            (IfFalse, IfFalse) => true,
            (IfEnd, IfEnd) => true,
            (Loop, Loop) => true,
            (LoopEnd, LoopEnd) => true,
            (LoopStart, LoopStart) => true,
            _ => false,
        }
    }
}

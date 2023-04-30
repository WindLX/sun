#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    Nil,
    True,
    False,
    And,
    Or,
    Not,
    Xor,

    // + - * / % ^ !
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Fac,

    // == != <= >= < > =
    Eq,
    NotEq,
    Le,
    Ge,
    Less,
    Greater,
    Assign,
    // () {} []
    ParL,
    ParR,
    CurL,
    CurR,
    SquL,
    SquR,
    // . , :
    Dot,
    Comma,
    Colon,

    // constants value
    Number(f64),

    // name
    Name(String),

    // string
    String(String),

    // end
    Eos,
}

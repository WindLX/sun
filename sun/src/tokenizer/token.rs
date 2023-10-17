/// Sun 的 最小语法单元
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    Nil,
    True,
    False,
    Return,
    Break,
    Continue,

    // if else
    If,
    Else,

    // loop
    Loop,

    // @ fn
    Import,
    DefFunction,

    // && || ! ^
    And,
    Or,
    Not,
    Xor,

    // + - * / %
    Add,
    Sub,
    Mul,
    Div,
    Mod,

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

    // . , : ; ::
    Dot,
    Comma,
    Colon,
    Semi,
    DoubleColon,

    // constants value
    Number(f64),

    // name
    Name(String),

    // string
    String(Vec<u8>),

    // end
    Eos,
}

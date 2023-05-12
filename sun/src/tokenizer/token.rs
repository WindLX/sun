/// Sun 的 最小语法单元
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    Nil,
    True,
    False,
    Return,

    // ' >>
    Clone,
    To,

    // ? ?? |
    If,
    Else,
    End,

    // $
    Loop,

    // @ # ##
    Import,
    DefFunction,
    DefClass,

    // && || ~ ^^
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

    // == ~= <= >= < > =
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

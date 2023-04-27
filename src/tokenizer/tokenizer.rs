use crate::tokenizer::token::Token;
use crate::utils::err::SunError;
use std::{
    io::{Bytes, Read},
    iter::Peekable,
    mem::replace,
    process,
};

/*
    词法分析器的结构体
    input: Peekable<Bytes<T>> 输入的sun脚本文件或标准输入
    ahead: Token 存储向前seek一个Token的结果，用以语法分析
*/
#[derive(Debug)]
pub struct Tokenizer<T: Read> {
    input: Peekable<Bytes<T>>,
    ahead: Token,
    line_num: u64,
}

impl<T: Read> Tokenizer<T> {
    /*
        Tokenizer 的构造函数
        para:
            input: File sun文件 / sun命令
            ahead: 初始值为 Token::EOS
    */
    pub fn new(input: T) -> Self {
        Tokenizer {
            input: input.bytes().peekable(),
            ahead: Token::Eos,
            line_num: 1,
        }
    }

    /*
        向前读取一个u8字符，如果读取为空则返回终止符 None
    */
    fn read_byte(&mut self) -> Option<u8> {
        self.input.next().and_then(|b| Some(b.unwrap()))
    }

    /*
        向前看一个u8字符并返回，不移动文件读取的指针
    */
    fn peek_byte(&mut self) -> Result<u8, SunError> {
        match self.input.peek() {
            Some(Ok(byte)) => Ok(*byte),
            Some(_) => Err(SunError::TokenizerError(
                "peek byte failed!".to_string(),
                self.line_num,
            )),
            None => Ok(b'\0'),
        }
    }

    /*
        获取长度为两个或一个字符的 Token
        para:
            next_char: u8 期望的下一个字符
            long_token: Token 期望的长度为2的Token
            short_token: Token 期望的长度为1的Token
        return:
            Token
    */
    fn read_2char(
        &mut self,
        next_char: u8,
        long_token: Token,
        short_token: Token,
    ) -> Result<Token, SunError> {
        if self.peek_byte()? == next_char {
            self.read_byte();
            Ok(long_token)
        } else {
            Ok(short_token)
        }
    }

    /*
        获取下一个变量名或者是否为sun的保留字
        para:
            first: u8 第一个字符
        return:
            Token: 返回变量名
    */
    fn read_name(&mut self, first: u8) -> Result<Token, SunError> {
        let mut s = String::new();
        s.push(first as char);

        loop {
            let ch = self.peek_byte()? as char;
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch as char);
                self.read_byte();
            } else {
                break;
            }
        }

        let res = match &s as &str {
            "false" => Token::False,
            "true" => Token::True,
            "nil" => Token::Nil,
            "and" => Token::And,
            "or" => Token::Or,
            "xor" => Token::Xor,
            "not" => Token::Not,
            _ => Token::Name(s),
        };
        Ok(res)
    }

    /*
        获取下一个数字
        para:
            first: char 第一个字符
        return:
            Token: 数字Token
    */
    fn read_number(&mut self, first: u8) -> Result<Token, SunError> {
        let mut n = (first - b'0') as i64;
        loop {
            let ch = self.peek_byte()?;
            if let Some(d) = char::to_digit(ch as char, 10) {
                n = n * 10 + d as i64;
                self.read_byte();
            } else if ch == b'.' {
                self.read_byte();
                return self.read_number_fraction(n);
            } else {
                break;
            }
        }

        // check if another .
        let fch = self.peek_byte()?;
        if (fch as char).is_alphabetic() {
            return Err(SunError::InvalidNumberError(
                "alphabetic in number".to_string(),
                self.line_num,
            ));
        } else if fch == b'.' {
            return Err(SunError::InvalidNumberError(
                "more than one `.` in number".to_string(),
                self.line_num,
            ));
        }

        Ok(Token::Number(n as f64))
    }

    /*
        获取下一个浮点数
        para:
            i: i64 浮点数的整数部分
        return:
            Token: 浮点数Token
    */
    fn read_number_fraction(&mut self, i: i64) -> Result<Token, SunError> {
        let mut n: i64 = 0;
        let mut x: f64 = 1.0;
        loop {
            let ch = self.peek_byte()?;
            if ch == b'.' {
                return Err(SunError::InvalidNumberError(
                    "more than one `.` in number".to_string(),
                    self.line_num,
                ));
            }
            if let Some(d) = char::to_digit(ch as char, 10) {
                n = n * 10 + d as i64;
                x *= 10.0;
                self.read_byte();
            } else {
                break;
            }
        }
        Ok(Token::Number(i as f64 + n as f64 / x))
    }

    /*
        跳过注释
    */
    fn read_comment(&mut self) {
        match self.read_byte() {
            None => (),
            Some(_) => loop {
                let ch = self.read_byte().unwrap();
                if ch == b'\n' || ch == b'\0' {
                    break;
                }
            },
        }
    }

    /*
        获取下一个 Token
        return:
            Token: 下一个Token
    */
    fn read_token(&mut self) -> Token {
        if let Some(ch) = self.read_byte() {
            let res: Result<Token, SunError> = match ch {
                b'\n' => {
                    self.line_num += 1;
                    Ok(self.read_token())
                }
                b'\r' | b'\t' | b' ' => Ok(self.read_token()),
                b'+' => Ok(Token::Add),
                b'*' => Ok(Token::Mul),
                b'%' => Ok(Token::Mod),
                b'^' => Ok(Token::Pow),
                b'(' => Ok(Token::ParL),
                b')' => Ok(Token::ParR),
                b'{' => Ok(Token::CurL),
                b'}' => Ok(Token::CurR),
                b'[' => Ok(Token::SquL),
                b']' => Ok(Token::SquR),
                b',' => Ok(Token::Comma),
                b'=' => self.read_2char(b'=', Token::Eq, Token::Assign),
                b'!' => self.read_2char(b'=', Token::NotEq, Token::Fac),
                b':' => Ok(Token::Colon),
                b'<' => self.read_2char(b'=', Token::Le, Token::Less),
                b'>' => self.read_2char(b'=', Token::Ge, Token::Greater),
                b'.' => match self.peek_byte() {
                    Ok(b'0'..=b'9') => self.read_number_fraction(0),
                    Ok(_) => Ok(Token::Dot),
                    Err(e) => Err(e),
                },
                b'/' => match self.peek_byte() {
                    Ok(b'/') => {
                        self.read_byte();
                        self.read_comment();
                        Ok(self.read_token())
                    }
                    Ok(_) => Ok(Token::Div),
                    Err(e) => Err(e),
                },
                b'0'..=b'9' => self.read_number(ch),
                b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.read_name(ch),
                b'\0' => Ok(Token::Eos),
                byte => Err(SunError::InvalidSymbolError(
                    format!("invalid char {}", byte).to_string(),
                    self.line_num,
                )),
            };
            match res {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(-1)
                }
            }
        } else {
            Token::Eos
        }
    }
}

/*
    Tokenizer 的迭代器
*/
impl<T: Read> Iterator for Tokenizer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ahead == Token::Eos {
            match self.read_token() {
                Token::Eos => None,
                t => Some(t),
            }
        } else {
            Some(replace(&mut self.ahead, Token::Eos))
        }
    }
}

/*
    Tokenizer 的peek方法
*/
impl<T: Read> Tokenizer<T> {
    pub fn peek(&mut self) -> Option<&Token> {
        if self.ahead == Token::Eos {
            self.ahead = self.read_token();
        }
        Some(&self.ahead)
    }

    pub fn line(&self) -> u64 {
        self.line_num
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::token::Token;

    use super::Tokenizer;
    use std::fs::File;

    #[test]
    fn test_token_1() {
        let mut tokenizer = Tokenizer::new(File::open("test/file/1.sun").unwrap());
        assert_eq!(tokenizer.next(), Some(Token::Name("print".to_string())));
        assert_eq!(tokenizer.peek(), Some(&Token::ParL));
        tokenizer.next();
        assert_eq!(tokenizer.next(), Some(Token::Number(10.2)));
        assert_eq!(tokenizer.next(), Some(Token::ParR));
    }
}

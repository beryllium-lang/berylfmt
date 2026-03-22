use num_bigint::{BigInt, BigUint};
use num_traits::Num;

#[derive(Debug, Clone)]
pub enum Token {
    IntLit(BigUint),
    FloatLit(f64),
    StrLit(String),
    CharLit(char),
    Ident(String),
    Var,
    If,
    Do,
    Colon,
    Comma,
    Dot,
    Func,
    OpenParen,
    CloseParen,
    Exit,
    Return,
    Plus,
    Minus,
    Aster,
    ForwSlash,
    BackSlash,
    Mod,
    Mut,
    LeftShift,
    Less,
    Greater,
    OpenCurly,
    CloseCurly
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::IntLit(v) => format!("INT_LIT({v})"),
            Token::FloatLit(v) => format!("FLOAT_LIT({v})"),
            Token::StrLit(v) => format!("STR_LIT({v})"),
            Token::CharLit(v) => format!("CHAR_LIT({v})"),
            Token::Ident(v) => format!("IDENT({v})"),
            _ => "NORMAL_TOKEN".into(),
        }
    }
}


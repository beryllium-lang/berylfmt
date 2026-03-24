use anyhow::{anyhow, Result};
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq)]
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
    CloseCurly,
    For,
    Repeat,
    Band,
    Bor,
    Bxor,
    Bxnot,
    As,
    Unsafe,
    And,
    Or,
    Not,
    Eq,
    Assign,
    NotEq,
    Nmsp,
    Typeof,
    Valat,
    Ptrto,
    Sizeof,
    PlusEq,
    MinusEq,
    AsterEq,
    ForwSlashEq,
    ModEq,
    PlusPlus,
    MinusMinus,
    Import,
    From,
    True,
    False,
    Nullptr,
    Class,
    Public,
    Enum,
    EOF,
}

impl Token {}

#[derive(Clone, Debug)]
pub struct TokenStream {
    data: Vec<Token>,
    cursor: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            cursor: 0,
        }
    }

    pub fn from_vec(v: Vec<Token>) -> Self {
        Self { data: v, cursor: 0 }
    }

    pub fn from_source(src: &str) -> Result<Self> {
        let tokens: Vec<Token> = Vec::new();

        Ok(TokenStream::from_vec(tokens))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn has_next(&self) -> bool {
        self.cursor < self.len() - 1
    }

    pub fn peek(&self) -> &Token {
        &self.data[self.cursor]
    }

    pub fn peek_n(&self, n: usize) -> &Token {
        &self.data[(self.cursor + n).min(self.len())]
    }

    pub fn advance(&mut self) {
        if self.has_next() {
            self.cursor += 1;
        }
    }

    pub fn advance_n(&mut self, n: usize) {
        self.cursor = (self.cursor + n).min(self.len());
    }

    pub fn try_consume(&mut self, tok: &Token) {
        if self.peek_n(1) == tok {
            self.advance();
        }
    }
}

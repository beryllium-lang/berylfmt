use anyhow::{anyhow, Result};
use num_bigint::BigUint;

use crate::utils;
use utils::StrSlice;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    IntLit(BigUint),
    FloatLit(f64),
    StrLit(StrSlice<'a>),
    CharLit(char),
    Ident(StrSlice<'a>),
    Whitespace(StrSlice<'a>),
    Comment(StrSlice<'a>),
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

impl Token<'_> {}

#[derive(Clone, Debug)]
pub struct TokenStream<'a> {
    source: StrSlice<'a>,
    data: Vec<Token<'a>>,
    cursor: usize,
}

impl<'a> TokenStream<'a> {
    fn from_vec(source: StrSlice<'a>, v: Vec<Token<'a>>) -> Self {
        Self { source: source, data: v, cursor: 0 }
    }

    pub fn from_source(src: StrSlice<'a>) -> Result<Self> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut idx: usize = 0;
        let mut idx_last = idx;

        while idx < src.len {
            if src[idx].is_ascii_whitespace() {
                let mut tok: StrSlice = todo!();
                idx += 1;

                loop {
                    tok.len += 1;
                    idx += 1;
                    if !src[idx].is_ascii_whitespace() {
                        break;
                    }
                }

                tokens.push(Token::Whitespace(tok));
            }
            idx_last = idx;

            if idx_last == idx {
                panic!("Lexer froze");
            }
        }

        Ok(TokenStream::from_vec(src, tokens))
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

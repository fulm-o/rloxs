use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two chracter tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ident(String),
    Literal { kind: LiteralKind },

    // Keywords
    And,
    Class,
    Else,
    Fn,
    For,
    Nil,
    If,
    Print,
    Or,
    Return,
    Super,
    This,
    Let,
    While,

    LineComment,

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_kind: TokenKind,
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.token_kind
        )
    }
}
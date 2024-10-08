use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
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

    // One or two character tokens
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
            "[{}:{}] {:?}",
            self.line, self.column, self.token_kind
        )
    }
}
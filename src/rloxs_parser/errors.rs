use std::{error::Error, fmt::Display};

use crate::syntax::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(UnexpectedToken),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(e) => write!(f, "{}", e)
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct UnexpectedToken {
    unexpected_token: TokenKind,
    expected_token: Option<TokenKind>,
    line: usize,
    column: usize,

}

impl UnexpectedToken {
    pub fn new(
    unexpected_token: TokenKind,
    expected_token: Option<TokenKind>,
    line: usize,
    column: usize,
    ) -> Self {
        UnexpectedToken { unexpected_token, expected_token, line, column }
    }
}

impl Display for UnexpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expected_token {
            Some(expected_token) => write!(
                f,
                "Unexpected token: {:?}, expected: {:?} at [{}:{}]",
                self.unexpected_token,
                expected_token,
                self.line,
                self.column
            ),
            None => write!(
                f,
                "Unexpected token: {:?} at [{}:{}]",
                self.unexpected_token,
                self.line,
                self.column,
            ),
        }
    }
}

impl From<UnexpectedToken> for ParseError {
    fn from(value: UnexpectedToken) -> Self {
        ParseError::UnexpectedToken(value)
    }
}
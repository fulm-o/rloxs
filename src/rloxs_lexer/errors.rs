use std::{error::Error, fmt::Display, num::ParseFloatError};

#[derive(Debug)]
pub struct UnexpectedChar {
    ch: char,
    line: usize,
    column: usize,
}

impl UnexpectedChar {
    pub fn new(ch: char, line: usize, column: usize) -> Self {
        Self { ch, line, column }
    }
}

impl Display for UnexpectedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character: {} at [{}:{}]", self.ch, self.line, self.column)
    }
}

impl Error for UnexpectedChar {}



#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar(UnexpectedChar),
    ParseFloatError(ParseFloatError),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedChar(e) => write!(f, "{}", e),
            LexerError::ParseFloatError(e) => write!(f, "{}", e),
        }
    }
}

impl From<UnexpectedChar> for LexerError {
    fn from(value: UnexpectedChar) -> Self {
        LexerError::UnexpectedChar(value)
    }
}

impl From<ParseFloatError> for LexerError {
    fn from(value: ParseFloatError) -> Self {
        LexerError::ParseFloatError(value)
    }
}
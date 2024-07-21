use std::fmt;

use crate::{rloxs_eval::EvalError, rloxs_lexer::LexerError, rloxs_parser::ParseError};

#[derive(Debug)]
pub enum CompileError {
    Lexer(LexerError),
    Parse(ParseError),
    Eval(EvalError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Lexer(e) => write!(f, "{}", e),
            CompileError::Parse(e) => write!(f, "{}", e),
            CompileError::Eval(e) => write!(f, "{}", e),
        }
    }
}

impl From<LexerError> for CompileError {
    fn from(value: LexerError) -> Self {
        CompileError::Lexer(value)
    }
}

impl From<ParseError> for CompileError {
    fn from(value: ParseError) -> Self {
        CompileError::Parse(value)
    }
}

impl From<EvalError> for CompileError {
    fn from(value: EvalError) -> Self {
        CompileError::Eval(value)
    }
}
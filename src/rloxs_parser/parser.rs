use crate::syntax::{Token, Expr, Operator};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
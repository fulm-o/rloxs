use crate::syntax::{Token, TokenKind, Expr, Operator};

use super::errors::{ParseError, UnexpectedToken};

#[derive(Debug)]
pub struct Parser {
    tokens:  Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Result<Self, ParseError> {
        Ok(Parser {tokens, pos: 0 })
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos-1]
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn eat(&mut self, token_kind: TokenKind) -> Result<(), ParseError> {
        let current_token = self.peek_token();

        if current_token.token_kind == token_kind {
            self.pos += 1;
            Ok(())
        }else {
            Err(
                UnexpectedToken::new(
                current_token.token_kind.clone(), //←clone()消したい
                Some(token_kind),
                current_token.line,
                current_token.column,
            ))?
        }
    }


    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_comparison()?;

        loop {
            match self.peek_token().token_kind {
                TokenKind::BangEqual => {
                    self.eat(TokenKind::BangEqual)?;
                },
                TokenKind::EqualEqual => {

                },
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }


}
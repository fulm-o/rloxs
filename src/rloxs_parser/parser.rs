use crate::syntax::{Expr, Operator, OperatorKind, Token, TokenKind};

use super::errors::{ParseError, UnexpectedToken};

#[derive(Debug)]
pub struct Parser {
    tokens:  Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
                tokens: tokens
                    .into_iter()
                    .filter(|t| t.token_kind != TokenKind::LineComment)
                    .collect(),
                pos: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.current()
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos.saturating_sub(1)]
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &self.tokens[self.tokens.len() - 1] // posが範囲外なら最後のトークン(Eof)を返す
        } else {
            &self.tokens[self.pos]
        }
    }

    fn eat(&mut self, token_kind: TokenKind) -> Result<(), ParseError> {
        let current_token = self.peek();

        if current_token.token_kind == token_kind {
            self.advance();
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
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {

        if let TokenKind::Ident(ident) = self.peek().token_kind.clone() {
            self.eat(TokenKind::Ident(ident.clone()))?;
            match self.peek().token_kind {
                TokenKind::Equal => {
                    self.eat(TokenKind::Equal)?;

                    let assign_node = Expr::Assign {
                        name: ident.clone(),
                        expr: Box::new(self.logic_or()?),
                    };
                    Ok(assign_node)
                },
                _ => Ok(self.logic_or()?)
            }

        }else {
            Ok(self.logic_or()?)
        }
    }

    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.logic_and()?;

        'l: loop {
            match self.peek().token_kind {
                TokenKind::Or => {
                    self.eat(TokenKind::Or)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.logic_and()?),
                    }
                },
                _ => break 'l,
            }
        }

        Ok(node)
    }

    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_equality()?;

        'l: loop {
            match self.peek().token_kind {
                TokenKind::And => {
                    self.eat(TokenKind::And)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_equality()?),
                    }
                },
                _ => break 'l,
            }
        }

        Ok(node)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_comparison()?;

        loop {
            match self.peek().token_kind {
                TokenKind::BangEqual => {
                    self.eat(TokenKind::BangEqual)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_comparison()?)
                    }
                },
                TokenKind::EqualEqual => {
                    self.eat(TokenKind::EqualEqual)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_comparison()?),
                    }
                },
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_term()?;

        loop {
            match self.peek().token_kind {
                TokenKind::Less => {
                    self.eat(TokenKind::Less)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_term()?)
                    }
                },
                TokenKind::LessEqual => {
                    self.eat(TokenKind::EqualEqual)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_term()?),
                    }
                },
                TokenKind::Greater => {
                    self.eat(TokenKind::Greater)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_term()?),
                    }
                },
                TokenKind::GreaterEqual => {
                    self.eat(TokenKind::GreaterEqual)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_term()?),
                    }
                },
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_factor()?;

        loop {
            match self.peek().token_kind {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_factor()?)
                    }
                },
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_factor()?),
                    }
                },
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.parse_unary()?;

        loop {
            match self.peek().token_kind {
                TokenKind::Star => {
                    self.eat(TokenKind::Star)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_comparison()?)
                    }
                },
                TokenKind::Slash => {
                    self.eat(TokenKind::Slash)?;
                    node = Expr::BinaryOp {
                        left: Box::new(node),
                        operator: token_to_operator(self.previous())?,
                        right: Box::new(self.parse_comparison()?),
                    }
                },
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().token_kind {
            TokenKind::Bang => {
                self.eat(TokenKind::Bang)?;
                Ok(Expr::UnaryOp {
                    operator: token_to_operator(self.previous())?,
                    operand: Box::new(self.parse_unary()?),
                })
            },
            TokenKind::Minus => {
                self.eat(TokenKind::Minus)?;
                Ok(Expr::UnaryOp {
                    operator: token_to_operator(self.previous())?,
                    operand: Box::new(self.parse_unary()?),
                })
            },
            _ => self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let current_token = self.peek().clone();

        match &current_token.token_kind {
            TokenKind::Literal { kind } => {
                self.eat(TokenKind::Literal { kind: kind.clone() })?;
                Ok(Expr::Literal { kind: kind.clone() })
            },
            TokenKind::LeftParen => {
                self.eat(TokenKind::LeftParen)?;
                let node = Expr::Grouping(Box::new(self.parse_expression()?));
                self.eat(TokenKind::RightParen)?;
                Ok(node)
            },
            TokenKind::Ident(ident) => {
                self.eat(TokenKind::Ident(ident.to_string()))?;

                Ok(Expr::Variable(ident.to_string()))
            },
            _ => Err(
                UnexpectedToken::new(
                    current_token.token_kind.clone(),
                    None,
                    current_token.line,
                    current_token.column
                )
            )?

        }
    }


}

fn token_to_operator(token: &Token) -> Result<Operator, ParseError> {
    let op_kind = match token.token_kind {
        TokenKind::And => OperatorKind::And,
        TokenKind::Or => OperatorKind::Or,
        TokenKind::Bang => OperatorKind::Not,
        TokenKind::BangEqual => OperatorKind::NotEqual,
        TokenKind::EqualEqual => OperatorKind::Equal,
        TokenKind::Less => OperatorKind::Less,
        TokenKind::LessEqual => OperatorKind::LessEqual,
        TokenKind::Greater => OperatorKind::Greater,
        TokenKind::GreaterEqual => OperatorKind::GreaterEqual,
        TokenKind::Plus => OperatorKind::Add,
        TokenKind::Minus => OperatorKind::Subtract,
        TokenKind::Star => OperatorKind::Multiply,
        TokenKind::Slash => OperatorKind::Divide,
        _ => {
            Err(UnexpectedToken::new(
                token.token_kind.clone(),
                None,
                token.line,
                token.column,
            ))?
        },
    };

    Ok(Operator { op_kind, pos: token.pos, line: token.line, column: token.column })
}
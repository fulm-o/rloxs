use crate::syntax::{token::LiteralKind, Token, TokenKind};

use super::parser::Parser;

#[test]
fn feature() {
    let tokens = vec![
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(1.0)
            },
            pos: 0,
            line: 1,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Plus,
            pos: 2,
            line: 1,
            column: 2,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(2.0)
            },
            pos: 4,
            line: 1,
            column: 4,
        },
        Token {
            token_kind: TokenKind::Star,
            pos: 6,
            line: 1,
            column: 6,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(3.0)
            },
            pos: 8,
            line: 1,
            column: 8,
        },
    ];

    let mut parser = Parser::new(tokens).unwrap();
    let ast = parser.parse_expression();
}
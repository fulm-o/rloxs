
use crate::syntax::token::{LiteralKind, Token, TokenKind};

use super::*;

fn test_helper(input: &str) -> Vec<Token> {
    Lexer::new(input).lex().unwrap()
}

#[test]
fn lex_new_line_and_space() {
    let tokens = test_helper("+-*/;()\n<>=   .");

    let expect_tokens = vec![
        Token { token_kind: TokenKind::Plus, pos: 0, line: 1, column: 0 },
        Token { token_kind: TokenKind::Minus, pos: 1, line: 1, column: 1 },
        Token { token_kind: TokenKind::Star, pos: 2, line: 1, column: 2 },
        Token { token_kind: TokenKind::Slash, pos: 3, line: 1, column: 3 },
        Token { token_kind: TokenKind::Semicolon, pos: 4, line: 1, column: 4 },
        Token { token_kind: TokenKind::LeftParen, pos: 5, line: 1, column: 5 },
        Token { token_kind: TokenKind::RightParen, pos: 6, line: 1, column: 6 },
        /* 改行 */
        Token { token_kind: TokenKind::Less, pos: 8, line: 2, column: 0 },
        Token { token_kind: TokenKind::GreaterEqual, pos: 9, line: 2, column: 1 },
        /* スペース×3 */
        Token { token_kind: TokenKind::Dot, pos: 14, line: 2, column: 6 },
    ];

    assert_eq!(tokens, expect_tokens);
}

#[test]
fn lex_string() {
    let tokens = test_helper(r#""String"+
"String2""#);

    let expect_tokens = vec![
        Token { token_kind: TokenKind::Literal {
            kind: LiteralKind::String("String".to_string()) },
            pos: 0,
            line: 1,
            column: 0,
        },
        Token { token_kind: TokenKind::Plus,
            pos: 8,
            line: 1,
            column: 8,
        },
        /* 改行 */
        Token { token_kind: TokenKind::Literal {
            kind: LiteralKind::String("String2".to_string()) },
            pos: 10,
            line: 2,
            column: 0,
        },
    ];

    assert_eq!(tokens, expect_tokens);
}

#[test]
fn lex_line_comment() {
    let tokens = test_helper("1 + 1 // nice comment\n2 * 3");

    let expect_tokens = vec![
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
                kind: LiteralKind::Number(1.0)
            },
            pos: 4,
            line: 1,
            column: 4,
        },
        Token {
            token_kind: TokenKind::LineComment,
            pos: 6,
            line: 1,
            column: 6,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(2.0)
            },
            pos: 22,
            line: 2,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Star,
            pos: 24,
            line: 2,
            column: 2,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(3.0)
            },
            pos: 26,
            line: 2,
            column: 4,
        },
    ];

    assert_eq!(tokens, expect_tokens);
}

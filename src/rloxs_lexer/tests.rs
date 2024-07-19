
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
        Token { token_kind: TokenKind::Eof, pos: 15, line: 2, column: 7 },
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
        Token { token_kind: TokenKind::Eof,
            pos: 19,
            line: 2,
            column: 9,
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
        Token {
            token_kind: TokenKind::Eof,
            pos: 27,
            line: 2,
            column: 5,
        },
    ];

    assert_eq!(tokens, expect_tokens);
}

#[test]
fn lex_hello_world() {
    let tokens = test_helper(r#"
fn func() {
    print "Hello, World!";
}
"#.trim());

    let expect_tokens = vec![
        Token {
            token_kind: TokenKind::Fn,
            pos: 0,
            line: 1,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Ident("func".to_string()),
            pos: 3,
            line: 1,
            column: 3,
        },
        Token {
            token_kind: TokenKind::LeftParen,
            pos: 7,
            line: 1,
            column: 7,
        },
        Token {
            token_kind: TokenKind::RightParen,
            pos: 8,
            line: 1,
            column: 8,
        },
        Token {
            token_kind: TokenKind::LeftBrace,
            pos: 10,
            line: 1,
            column: 10,
        },
        Token {
            token_kind: TokenKind::Print,
            pos: 16,
            line: 2,
            column: 4,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::String("Hello, World!".to_string())
            },
            pos: 22,
            line: 2,
            column: 10,
        },
        Token {
            token_kind: TokenKind::Semicolon,
            pos: 37,
            line: 2,
            column: 25,
        },
        Token {
            token_kind: TokenKind::RightBrace,
            pos: 39,
            line: 3,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Eof,
            pos: 40,
            line: 3,
            column: 1,
        },
    ];

    assert_eq!(tokens, expect_tokens);
}

#[test]
fn lex_various_tokens() {
    let tokens = test_helper(r#"
let x = 42;
let y = x + 1.23;
"#.trim());

    let expect_tokens = vec![
        Token {
            token_kind: TokenKind::Let,
            pos: 0,
            line: 1,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Ident("x".to_string()),
            pos: 4,
            line: 1,
            column: 4,
        },
        Token {
            token_kind: TokenKind::Equal,
            pos: 6,
            line: 1,
            column: 6,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(42.0),
            },
            pos: 8,
            line: 1,
            column: 8,
        },
        Token {
            token_kind: TokenKind::Semicolon,
            pos: 10,
            line: 1,
            column: 10,
        },
        Token {
            token_kind: TokenKind::Let,
            pos: 12,
            line: 2,
            column: 0,
        },
        Token {
            token_kind: TokenKind::Ident("y".to_string()),
            pos: 16,
            line: 2,
            column: 4,
        },
        Token {
            token_kind: TokenKind::Equal,
            pos: 18,
            line: 2,
            column: 6,
        },
        Token {
            token_kind: TokenKind::Ident("x".to_string()),
            pos: 20,
            line: 2,
            column: 8,
        },
        Token {
            token_kind: TokenKind::Plus,
            pos: 22,
            line: 2,
            column: 10,
        },
        Token {
            token_kind: TokenKind::Literal {
                kind: LiteralKind::Number(1.23),
            },
            pos: 24,
            line: 2,
            column: 12,
        },
        Token {
            token_kind: TokenKind::Semicolon,
            pos: 28,
            line: 2,
            column: 16,
        },
        Token {
            token_kind: TokenKind::Eof,
            pos: 29,
            line: 2,
            column: 17,
        },
    ];

    assert_eq!(tokens, expect_tokens);
}

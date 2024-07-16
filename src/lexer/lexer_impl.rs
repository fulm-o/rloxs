

use crate::syntax::token::{LiteralKind, Token, TokenKind};

use super::error::LexerError;

pub struct Lexer {
    input: Vec<char>,
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect::<Vec<char>>(),
            tokens: vec![],
            pos: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            let token = self.next_token()?;
            self.tokens.push(token);
        }

        Ok(self.tokens.to_owned())
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    //現在位置の文字を返却するメソッド
    fn peek_char(&self) -> Option<char> {
        if !self.is_at_end() {
            Some(self.input[self.pos])
        }else {
            None
        }
    }

    //現在の文字を返却し、位置を進めておくメソッド
    fn next_char(&mut self) -> Option<char> {
        if !self.is_at_end() {
            let ch = self.input[self.pos];
            self.pos += 1;
            self.column += 1;
            Some(ch)
        }else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }

            if ch == '\n' {
                self.next_char();

                self.line += 1;
                self.column = 0;

            } else if ch == '\r' {
                self.next_char();

                //二回目のnext_char()は境界チェックができないためif let 文
                if let Some('\n') = self.peek_char() {
                    self.next_char();
                    self.line += 1;
                    self.column = 0;
                }
            }else {
                self.next_char();
            }
        }
    }

    fn match_next_char(&mut self, expect_char: char) -> bool {
        if self.is_at_end() {
            false
        }else {
            self.input[self.pos] == expect_char
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        let pos = self.pos;
        let line = self.line;
        let column = self.column;

        let token_kind = match self.next_char() {
            Some(ch) => match ch {
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Star,
                    '(' => TokenKind::LeftParen,
                    ')' => TokenKind::RightParen,
                    '{' => TokenKind::LeftBrace,
                    '}' => TokenKind::RightBrace,
                    '.' => TokenKind::Dot,
                    ',' => TokenKind::Comma,
                    ';' => TokenKind::Semicolon,
                    '/' => {
                        if self.match_next_char('/') {
                            while let Some(ch) = self.next_char() {
                                if ch == '\n' {
                                    self.line += 1;
                                    self.column = 0;
                                    break;
                                }
                            }
                            TokenKind::LineComment
                        }else {
                            TokenKind::Slash
                        }
                    },
                    '=' => {
                        if self.match_next_char('=') {
                            //文字を次に進める。返り値は利用しない。
                            //self.posとself.columnをインクリメントするため
                            self.next_char();
                            TokenKind::EqualEqual
                        }else {
                            TokenKind::Equal
                        }
                    },
                    '!' => {
                        if self.match_next_char('=') {
                            self.next_char();
                            TokenKind::BangEqual
                        }else {
                            TokenKind::Bang
                        }
                    },
                    '<' => {
                        if self.match_next_char('=') {
                            self.next_char();
                            TokenKind::LessEqual
                        }else {
                            TokenKind::Less
                        }
                    },
                    '>' => {
                        if self.match_next_char('=') {
                            self.next_char();
                            TokenKind::GreaterEqual
                        }else {
                            TokenKind::Greater
                        }
                    },
                    '"' => {
                        let mut str = String::new();
                        while let Some(ch) = self.next_char() {
                            if ch == '"' {
                                break;
                            }else {
                                str.push(ch);
                            }
                        }

                        TokenKind::Literal { kind: LiteralKind::String(str) }
                    },
                    ('0'..='9') => {
                        let integer = self.read_integer(ch);
                        match integer.parse::<f64>() {
                            Ok(f) => TokenKind::Literal { kind: LiteralKind::Number(f) },
                            Err(e) => Err(e)?,
                        }
                    },
                    _ => {
                        
                        todo!();
                    }
            },
            None => {
                TokenKind::Eof
            }
        };

        Ok(
            Token {
                token_kind,
                pos,
                line,
                column,
            }
        )
    }

    fn read_integer(&mut self, first_char: char) -> String {
        match first_char {
            ('0'..='9') => {
                let mut result = vec![];
                result.push(first_char);

                while let Some('0'..='9') | Some('.') = self.peek_char() {
                    let num = self.peek_char().unwrap();
                    result.push(num);
                }

                result.iter().collect()
            },
            _ => unreachable!(),
        }
    }
}

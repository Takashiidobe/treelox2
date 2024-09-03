use crate::error;
use crate::token::{Location, Object, Token, TokenType};
use std::collections::HashMap;
use std::process::exit;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.as_bytes().to_vec(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            r#type: TokenType::Eof,
            loc: Location {
                lexeme: "EOF".to_string(),
                line: self.line,
            },
            literal: None,
        });

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        use TokenType::*;

        match c {
            b'(' => self.add_token(LeftParen, None),
            b')' => self.add_token(RightParen, None),
            b'{' => self.add_token(LeftBrace, None),
            b'}' => self.add_token(RightBrace, None),
            b',' => self.add_token(Comma, None),
            b'.' => self.add_token(Dot, None),
            b'-' => self.add_token(Minus, None),
            b'+' => self.add_token(Plus, None),
            b';' => self.add_token(Semicolon, None),
            b'*' => self.add_token(Star, None),
            b'!' => {
                if self.r#match(b'=') {
                    self.add_token(BangEqual, None)
                } else {
                    self.add_token(Bang, None)
                }
            }
            b'=' => {
                if self.r#match(b'=') {
                    self.add_token(EqualEqual, None)
                } else {
                    self.add_token(Equal, None)
                }
            }
            b'<' => {
                if self.r#match(b'=') {
                    self.add_token(LessEqual, None)
                } else {
                    self.add_token(Less, None)
                }
            }
            b'>' => {
                if self.r#match(b'=') {
                    self.add_token(GreaterEqual, None)
                } else {
                    self.add_token(Greater, None)
                }
            }
            b'/' => {
                if self.r#match(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash, None);
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(),
            _ => error(self.line, &format!("Unexpected character: {}", c)),
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        self.source[self.current + 1]
    }

    fn r#match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Object::Number(
                String::from_utf8_lossy(&self.source[self.start..self.current])
                    .parse()
                    .unwrap(),
            )),
        )
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "unterminated string.");
            exit(65);
        }

        self.advance();

        let val = String::from_utf8_lossy(&self.source[(self.start + 1)..(self.current - 1)]);
        self.add_token(TokenType::String, Some(Object::String(val.to_string())))
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let val = String::from_utf8_lossy(&self.source[self.start..self.current]).to_string();
        let token_type = self.keywords.get(&val);
        match token_type {
            Some(matching_type) => self.add_token(matching_type.clone(), None),
            None => self.add_token(TokenType::Identifier, Some(Object::String(val))),
        }
    }

    fn advance(&mut self) -> u8 {
        let res = self.source[self.current];
        self.current += 1;
        res
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = self.source[self.start..self.current].to_vec();
        self.tokens.push(Token {
            r#type: token_type,
            loc: Location {
                lexeme: String::from_utf8_lossy(&text).to_string(),
                line: self.line,
            },
            literal,
        });
    }
}

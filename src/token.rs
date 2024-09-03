use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: Option<Object>,
    pub loc: Location,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:?} {} on line {}",
            self.r#type, self.loc.lexeme, self.loc.line
        ))
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    String(String),
    Number(f64),
    Identifier(String),
    Bool(bool),
    #[default]
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(b) => *b,
            Object::Nil => false,
            _ => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(str) => f.write_str(str),
            Object::Number(num) => f.write_str(&num.to_string()),
            Object::Identifier(ident) => f.write_str(ident),
            Object::Bool(b) => f.write_str(&b.to_string()),
            Object::Nil => f.write_str("nil"),
        }
    }
}

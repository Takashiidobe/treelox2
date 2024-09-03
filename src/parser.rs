use crate::{
    errors::Errors,
    expr::Expr,
    token::{Object, Token, TokenType},
};

#[derive(Default, Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Errors,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let res = self.expression();
        if self.errors.had_error {
            None
        } else {
            Some(res)
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut left = self.comparison();
        while self.r#match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison());
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right,
            }
        }

        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.term();

        while self.r#match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term());
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right,
            }
        }

        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while self.r#match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor());
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right,
            };
        }

        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.unary();

        while self.r#match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right,
            };
        }

        left
    }

    fn unary(&mut self) -> Expr {
        if self.r#match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            return Expr::Unary { operator, right };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.r#match(&[TokenType::False]) {
            return Expr::Literal {
                value: Object::Bool(false),
            };
        }
        if self.r#match(&[TokenType::True]) {
            return Expr::Literal {
                value: Object::Bool(true),
            };
        }
        if self.r#match(&[TokenType::Nil]) {
            return Expr::Literal { value: Object::Nil };
        }

        if self.r#match(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal {
                value: self.previous().clone().literal.unwrap(),
            };
        }

        if self.r#match(&[TokenType::LeftParen]) {
            let expr = Box::new(self.expression());
            self.consume(&TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping { expr };
        }

        self.errors
            .error_token(&self.peek().clone(), "Expect expression.");
        panic!("Couldn't parse {:?}", self.peek());
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance().clone());
        }

        self.errors.error_token(&self.peek().clone(), msg);
        None
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn r#match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == *token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

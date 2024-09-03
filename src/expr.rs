use crate::token::{Object, Token};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<R, E> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, E>;
    fn visit_grouping_expr(&self, expr: &Expr) -> Result<R, E>;
    fn visit_literal_expr(&self, value: &Object) -> Result<R, E>;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<R, E>;
}

impl Expr {
    pub fn accept<R, E>(&self, visitor: &dyn Visitor<R, E>) -> Result<R, E> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expr } => visitor.visit_grouping_expr(expr),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> Result<String, ()> {
        Ok(expr.accept(self)?)
    }

    fn parenthesize(&self, name: String, exprs: &[&Expr]) -> Result<String, ()> {
        let mut r = String::new();
        r.push('(');
        r.push_str(&name);
        for expr in exprs {
            r.push(' ');
            r.push_str(&expr.accept(self)?);
        }
        r.push(')');
        Ok(r)
    }
}

impl Visitor<String, ()> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<String, ()> {
        self.parenthesize(operator.loc.lexeme.clone(), &[left, right])
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<String, ()> {
        self.parenthesize("group".to_string(), &[expr])
    }

    fn visit_literal_expr(&self, value: &Object) -> Result<String, ()> {
        Ok(value.to_string())
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<String, ()> {
        self.parenthesize(operator.loc.lexeme.clone(), &[right])
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Location, TokenType};

    use super::*;

    #[test]
    fn test_multiplication() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    r#type: TokenType::Minus,
                    literal: None,
                    loc: Location {
                        line: 1,
                        lexeme: "-".to_string(),
                    },
                },
                right: Box::new(Expr::Literal {
                    value: Object::Number(123.0),
                }),
            }),
            operator: Token {
                r#type: TokenType::Star,
                literal: None,
                loc: Location {
                    lexeme: "*".to_string(),
                    line: 1,
                },
            },
            right: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Literal {
                    value: Object::Number(45.67),
                }),
            }),
        };

        let result = AstPrinter.print(expression).unwrap();

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}

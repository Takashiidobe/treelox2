use crate::{
    expr::{Expr, Visitor},
    token::{Object, Token, TokenType},
    InterpreterError,
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, InterpreterError> {
        expr.accept(self)
    }
}

impl Visitor<Object, InterpreterError> for Interpreter {
    fn visit_binary_expr(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, InterpreterError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.r#type {
            TokenType::Minus => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::Slash => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::Star => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::Plus => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l.clone() + r)),
                _ => Err(InterpreterError::InvalidStringOrNumberOperands(left, right)),
            },
            TokenType::Greater => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l > r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l >= r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::Less => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l < r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::LessEqual => match (&left, &right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l <= r)),
                _ => Err(InterpreterError::InvalidNumberOperands(left, right)),
            },
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => Err(InterpreterError::InvalidOperatorError(
                left,
                operator.clone(),
                right,
            )),
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<Object, InterpreterError> {
        self.evaluate(expr)
    }

    fn visit_literal_expr(&self, value: &Object) -> Result<Object, InterpreterError> {
        Ok(value.clone())
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
        let right = self.evaluate(right)?;

        match operator.r#type {
            TokenType::Minus => match right {
                Object::Number(num) => Ok(Object::Number(-num)),
                _ => Err(InterpreterError::InvalidNumberOperand(right)),
            },
            TokenType::Bang => Ok(Object::Bool(right.is_truthy())),
            _ => Err(InterpreterError::InvalidNumberOperand(right)),
        }
    }
}

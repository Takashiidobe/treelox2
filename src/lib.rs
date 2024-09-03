use expr::AstPrinter;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use thiserror::Error;
use token::{Object, Token};

pub mod errors;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn run(source: &str) -> Result<(), InterpreterError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse();
    let mut interpreter = Interpreter;

    match expr {
        Some(e) => interpreter.interpret(&e)?,
        None => {
            panic!("Error while scanning tokens");
        }
    }
    Ok(())
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

pub fn report(line: usize, loc: &str, msg: &str) {
    eprintln!("[line: {line}] Error {loc}: {msg}");
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Operand {0} must be a number.")]
    InvalidNumberOperand(Object),
    #[error("Operands {0} and {1} must be a number.")]
    InvalidNumberOperands(Object, Object),
    #[error("Operands {0} and {1} must be a number or string.")]
    InvalidStringOrNumberOperands(Object, Object),
    #[error("Operator {1} cannot be applied to {1} and {2}.")]
    InvalidOperatorError(Object, Token, Object),
}

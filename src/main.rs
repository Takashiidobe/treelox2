use std::env::args;
use std::process::exit;

use treelox2::{lox::Lox, InterpreterError};

fn main() -> Result<(), InterpreterError> {
    let arguments: Vec<_> = args().collect();
    let mut lox = Lox::default();

    match arguments.len() {
        1 => {
            lox.run_prompt()?;
        }
        2 => {
            lox.run_file(&arguments[1])?;
        }
        _ => {
            eprintln!("Usage: treelox2 [script]");
            exit(64);
        }
    }
    Ok(())
}

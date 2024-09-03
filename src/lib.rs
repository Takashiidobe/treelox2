use expr::AstPrinter;
use parser::Parser;
use scanner::Scanner;

pub mod errors;
pub mod expr;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse();

    match expr {
        Some(expr) => {
            println!("{}", AstPrinter.print(expr));
        }
        None => {
            panic!("Error while scanning tokens");
        }
    }
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

pub fn report(line: usize, loc: &str, msg: &str) {
    eprintln!("[line: {line}] Error {loc}: {msg}");
}

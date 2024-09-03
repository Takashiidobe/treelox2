use scanner::Scanner;

pub mod expr;
pub mod lox;
pub mod scanner;
pub mod token;

pub fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

pub fn report(line: usize, loc: &str, msg: &str) {
    eprintln!("[line: {line}] Error {loc}: {msg}");
}

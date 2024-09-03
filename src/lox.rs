use std::{fs::read_to_string, process::exit};

use crate::run;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn run_file(&mut self, path: &str) {
        run(&read_to_string(path).expect("Invalid file location"));
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut buf = String::default();
            let line = std::io::stdin().read_line(&mut buf);
            match line {
                Ok(0) => break,
                Ok(_) => run(&buf),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }
            if self.had_error {
                exit(65);
            }
        }
    }
}

use std::{fs::read_to_string, io, process::exit};

use crate::{run, InterpreterError};

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn run_file(&mut self, path: &str) -> Result<(), InterpreterError> {
        if let Err(e) = run(&read_to_string(path).expect("Invalid file location")) {
            eprintln!("{}", e);
            exit(70);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), InterpreterError> {
        loop {
            print!("> ");
            let mut buf = String::default();
            let line = io::stdin().read_line(&mut buf);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    if let Err(e) = run(&buf) {
                        self.had_error = true;
                        eprintln!("{}", e);
                    }
                }
                Err(e) => {
                    self.had_error = true;
                    eprintln!("{}", e);
                }
            }
        }
        Ok(())
    }
}

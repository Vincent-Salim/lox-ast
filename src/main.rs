mod token;
mod token_type;

// mod ast_printer;
// use ast_printer::*;

mod expr;

mod interpreter;
use interpreter::*;

mod object;

mod parser;
use parser::*;

mod error;
use error::*;

mod scanner;
use scanner::*;

use std::env::args;
use std::io::{self, BufRead, Write, stdout};

pub fn main() {
    let args: Vec<String> = args().collect();
    let lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter {},
        }
    }
    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err() {
            // Ignore: error was reported elsewhere
            std::process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line);
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    pub fn run(&self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            None => {}
            Some(expr) => {
                self.interpreter.interpret(&expr);
                // let printer = AstPrinter {};
                // println!("Ast printer:\n{}", printer.print(&expr)?);
            }
        }

        Ok(())
    }
}

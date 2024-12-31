mod ast_printer;
mod environment;
mod expressions;
mod interpreter;
mod parser;
mod reporter;
mod scanner;
mod statements;
mod token;
use interpreter::Interpreter;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

use crate::reporter::StdoutReporter;

struct Lux;

impl Lux {
    fn run_file(file_path: &str) -> Result<(), std::io::Error> {
        let program = fs::read_to_string(file_path)?;
        let mut interpreter = interpreter::Interpreter::new();
        Self::run(&program, &mut interpreter);
        Ok(())
    }

    fn run_prompt() -> Result<(), std::io::Error> {
        let mut interpreter = interpreter::Interpreter::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut buf = String::new();
            let _bytes = io::stdin().read_line(&mut buf)?;
            Self::run(&buf, &mut interpreter);
        }
    }

    fn run(source: &str, interpreter: &mut Interpreter) {
        let mut reporter = StdoutReporter::default();
        let mut scanner = scanner::Scanner::new(source, &mut reporter);
        scanner.scan_tokens();
        let tokens = scanner.into_tokens();
        let mut parser = parser::Parser::new(tokens, &mut reporter);
        let statements = parser.parse();
        match interpreter.interpret(&statements) {
            Ok(_) => {}
            Err(message) => println!("error in interpreter: {}", message),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        Lux::run_file(&args[1])?;
    } else {
        Lux::run_prompt()?;
    }
    Ok(())
}

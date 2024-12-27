mod expressions;
mod parser;
mod reporter;
mod scanner;
mod token;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

use crate::expressions::Accept;
use crate::reporter::StdoutReporter;

struct Lux;

impl Lux {
    fn run_file(file_path: &str) -> Result<(), std::io::Error> {
        let program = fs::read_to_string(file_path)?;
        Self::run(&program);
        Ok(())
    }

    fn run_prompt() -> Result<(), std::io::Error> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut buf = String::new();
            let _bytes = io::stdin().read_line(&mut buf)?;
            Self::run(&buf);
        }
    }

    fn run(source: &str) {
        let reporter = StdoutReporter::default();
        let mut scanner = scanner::Scanner::new(source, reporter);
        scanner.scan_tokens();
        let tokens = scanner.into_tokens();
        let reporter = StdoutReporter::default(); // TODO: avoid initializing twice
        let mut parser = parser::Parser::new(tokens, reporter);
        let expression = parser.parse();
        let visitor = expressions::AstPrinter {};
        let printed = expression.accept(&visitor);
        println!("{}", printed);
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

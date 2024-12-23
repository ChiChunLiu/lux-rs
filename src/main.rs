mod reporter;
mod scanner;
mod token;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

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
        let mut reporter = StdoutReporter::default();
        let mut scanner = scanner::Scanner::new(source, &mut reporter);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token.to_string())
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

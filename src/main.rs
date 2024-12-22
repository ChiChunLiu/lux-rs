use std::env;
use std::fs;
use std::io;
use std::io::Write;

struct Lux;

impl Lux {
    fn run_file(file_path: &str) -> Result<(), std::io::Error> {
        let program = fs::read_to_string(file_path)?;
        println!("{program}");
        Ok(())
    }
    fn run_prompt() -> Result<(), std::io::Error> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut buf = String::new();
            let _bytes = io::stdin().read_line(&mut buf)?;
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

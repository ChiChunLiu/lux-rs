use crate::token::{Token, TokenType};

pub trait Reporter {
    fn scanner_error(&mut self, line: usize, message: &str);
    fn parser_error(&mut self, token: &Token, message: &str);
    fn report(&mut self, line: usize, error_where: &str, message: &str);
}

#[derive(Default)]
pub struct StdoutReporter {
    had_error: bool,
}

impl Reporter for StdoutReporter {
    fn scanner_error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    fn parser_error(&mut self, token: &Token, message: &str) {
        match token.token_type {
            TokenType::EOF => self.report(token.line, " at end", message),
            _ => self.report(token.line, &format!("at '{}'", token.lexeme), message),
        }
    }
    fn report(&mut self, line: usize, error_where: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, error_where, message);
        self.had_error = true;
    }
}

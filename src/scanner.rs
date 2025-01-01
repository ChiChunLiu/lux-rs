use crate::reporter::Reporter;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub reporter: &'a mut dyn Reporter,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, reporter: &'a mut dyn Reporter) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            reporter,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn match_char(&mut self, c: char) -> bool {
        let matched = !self.is_at_end() && self.source.as_bytes()[self.current] as char == c;
        if matched {
            self.current += 1;
        }
        matched
    }

    fn peek(&self) -> Option<char> {
        (!self.is_at_end()).then(|| self.source.as_bytes()[self.current] as char)
    }

    fn peek_next(&self) -> Option<char> {
        (self.current + 1 < self.source.len())
            .then(|| self.source.as_bytes()[self.current + 1] as char)
    }

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '"' => break,
                'n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }
        if self.is_at_end() {
            self.reporter.scanner_error(self.line, "string not closed");
        }
        self.advance();
        let string_literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(string_literal))
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_'
    }

    fn number(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        let digits = &self.source[self.start..self.current];
        self.add_token(TokenType::Number(
            digits.parse::<f64>().expect("failed to parse float"),
        ))
    }

    fn identifier(&mut self) {
        while self.peek().map_or(false, Self::is_alphanumeric) {
            self.advance();
        }
        let token_type = match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => {
                let message = format!("encountered unexpected character: {}", c);
                self.reporter.scanner_error(self.line, &message)
            }
        };
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EndOfFile,
            lexeme: "".to_string(),
            line: self.line,
        });
    }

    /// Obtain the scanned tokens and consume the scanner when scanning is finished
    ///
    /// # Returns
    /// Resulting tokens scanned from the source program
    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::StdoutReporter;

    #[test]
    fn test_scanner() {
        let source = "var a = 3.1;";
        let mut reporter = StdoutReporter::default();
        let mut scanner = Scanner::new(source, &mut reporter);
        scanner.scan_tokens();
        let tokens = scanner.into_tokens();
        assert_eq!(
            tokens,
            [
                Token {
                    token_type: TokenType::Var,
                    lexeme: "var".to_string(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "a".to_string(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Equal,
                    lexeme: "=".to_string(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Number(3.1),
                    lexeme: "3.1".to_string(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Semicolon,
                    lexeme: ";".to_string(),
                    line: 0
                },
                Token {
                    token_type: TokenType::EndOfFile,
                    lexeme: "".to_string(),
                    line: 0
                }
            ]
        );
    }
}

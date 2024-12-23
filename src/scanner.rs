use crate::reporter::Reporter;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token<'a>>,
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
        if self.is_at_end() || self.source.as_bytes()[self.current] as char != c {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source.as_bytes()[self.current] as char;
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source.as_bytes()[self.current + 1] as char;
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.reporter.error(self.line, "string not closed");
        }
        self.advance();
        let string_literal = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String(string_literal))
    }

    fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_'
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) && !self.is_at_end() {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) && !self.is_at_end() {
                self.advance();
            }
        }
        let digits = &self.source[self.start..self.current];
        self.add_token(TokenType::Number(
            digits.parse::<f64>().expect("failed to parse float"),
        ))
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
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

    fn add_token(&mut self, token_type: TokenType<'a>) {
        let lexeme = &self.source[self.start..self.current];
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            c if Self::is_digit(c) => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => {
                let message = format!("encountered unexpected character: {}", c);
                self.reporter.error(self.line, &message)
            }
        };
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            line: self.line,
        });
        &self.tokens
    }
}

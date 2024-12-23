use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
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
            _ => {}
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

use crate::expressions::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr};
use crate::reporter::Reporter;
use crate::token::{Token, TokenType};

// Grammar:
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub reporter: &'a mut dyn Reporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, reporter: &'a mut dyn Reporter) -> Self {
        Self {
            tokens,
            current: 0,
            reporter,
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn match_token_types(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Expr::Unary(Box::new(UnaryExpr { operator, right }))
        } else {
            self.primary().unwrap()
        }
    }

    // fn consume(&mut self, token_type: TokenType, message: &str) {
    //     if self.check(&token_type) {
    //         self.advance();
    //     } else {
    //         let token = self.peek().clone();
    //         self.reporter.parser_error(&token, message)
    //     }
    // }

    fn primary(&mut self) -> Result<Expr, &str> {
        match &self.peek().token_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Bool(false),
                })))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Bool(true),
                })))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Nil,
                })))
            }
            TokenType::Number(value) => {
                let v = *value; // copy to make borrow checker happy when calling advance below.
                self.advance();
                Ok(Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Number(v),
                })))
            }
            TokenType::String(value) => {
                let v = value.clone();
                self.advance();
                Ok(Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::String(v),
                })))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                let token = self.advance();
                match token.token_type {
                    TokenType::RightParen => Ok(Expr::Grouping(Box::new(GroupingExpr { expr }))),
                    _ => Err("Parsing error: expecting ')'"),
                }
            }
            _ => Err("parsing error"),
        }
    }
}

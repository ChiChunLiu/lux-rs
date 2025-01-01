use crate::expressions::{
    BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr, VarExpr,
};
use crate::reporter::Reporter;
use crate::statements::{ExprStmt, PrintStmt, Stmt, VarStmt};
use crate::token::{Token, TokenType};

pub struct ParseError {
    token: Token,
    message: String,
}
impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}
// Statement grammar:
// program        → declaration* EOF ;
// declaration    → varDecl
//                | statement ;
// statement      → exprStmt
//                | printStmt ;

// Expression grammar:
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
        !self.is_at_end() && &self.peek().token_type == token_type
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
        let matched = token_types.iter().any(|token_type| self.check(token_type));
        if matched {
            self.advance();
        }
        matched
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let token = self.peek().clone();
            Err(ParseError::new(token, message.to_string()))
        }
    }

    fn declaration(&mut self) -> Stmt {
        let nil_stub_stmt = Stmt::Expr(Box::new(ExprStmt {
            expr: Expr::Literal(Box::new(LiteralExpr {
                value: LiteralValue::Nil,
            })),
        }));
        if self.match_token_types(&[TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => stmt,
                Err(error) => {
                    self.synchronize();
                    self.reporter.parser_error(&error.token, &error.message);
                    nil_stub_stmt
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => stmt,
                Err(error) => {
                    self.synchronize();
                    self.reporter.parser_error(&error.token, &error.message);
                    nil_stub_stmt
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let initializer = if self.match_token_types(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(Box::new(VarStmt { name, initializer })))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token_types(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Box::new(PrintStmt { expr: value })))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expr(Box::new(ExprStmt { expr })))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
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
                let expr = self.expression()?;
                let token = self.advance();
                match token.token_type {
                    TokenType::RightParen => Ok(Expr::Grouping(Box::new(GroupingExpr { expr }))),
                    _ => Err(ParseError::new(
                        self.peek().clone(),
                        "Parsing error: expecting ')'".to_string(),
                    )),
                }
            }
            TokenType::Identifier => {
                let token = self.advance().clone();
                Ok(Expr::Variable(Box::new(VarExpr { name: token })))
            }
            _ => Err(ParseError::new(
                self.peek().clone(),
                "No other literal token types . Not reachable.".to_string(),
            )),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration())
        }
        statements
    }

    /// March forward until the beginning of the next statement. Used for
    /// recovering from an parser error.
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                break;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                break;
            }
            self.advance();
        }
    }
}

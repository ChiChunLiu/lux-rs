use crate::expressions::{
    Accept, BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr,
};
use crate::statements::Accept as StmtAccept;
use crate::statements::{Stmt, StmtVisitor};
use crate::token::TokenType;

pub struct Interpreter;
impl Interpreter {
    pub fn interpret(&self, statements: &Vec<Stmt>) -> Result<(), &'static str> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn execute(&self, stmt: &Stmt) -> Result<(), &'static str> {
        stmt.accept(self)
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<LiteralValue, &'static str> {
        expr.accept(self)
    }
    fn is_truthy(expr: &LiteralValue) -> bool {
        match &expr {
            LiteralValue::Nil => false,
            LiteralValue::Bool(value) => *value,
            _ => true,
        }
    }
}

impl StmtVisitor<Result<(), &'static str>> for Interpreter {
    fn visit_expr_stmt(&self, stmt: &crate::statements::ExprStmt) -> Result<(), &'static str> {
        self.evaluate(&stmt.expr)?;
        Ok(())
    }
    fn visit_print_stmt(&self, stmt: &crate::statements::PrintStmt) -> Result<(), &'static str> {
        let value = self.evaluate(&stmt.expr)?;
        println!("{}", value);
        Ok(())
    }
}

impl ExprVisitor<Result<LiteralValue, &'static str>> for Interpreter {
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<LiteralValue, &'static str> {
        let right = self.evaluate(&expr.right)?;
        match &expr.operator.token_type {
            TokenType::Minus => {
                if let LiteralValue::Number(n) = right {
                    Ok(LiteralValue::Number(-n))
                } else {
                    Err("negation can only act on a number")
                }
            }
            TokenType::Bang => Ok(LiteralValue::Bool(!Self::is_truthy(&right))),
            _ => Err("unary operation can only have operator '-' or '!'"),
        }
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<LiteralValue, &'static str> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        match &expr.operator.token_type {
            TokenType::Minus => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Number(v_left - v_right))
                } else {
                    Err("substraction can only act on a pair of numbers")
                }
            }
            TokenType::Slash => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Number(v_left / v_right))
                } else {
                    Err("negation can only act on a pair of numbers")
                }
            }
            TokenType::Star => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Number(v_left * v_right))
                } else {
                    Err("negation can only act on a pair of numbers")
                }
            }
            TokenType::Less => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Bool(v_left < v_right))
                } else {
                    Err("< can only act on a pair of numbers")
                }
            }
            TokenType::Greater => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Bool(v_left > v_right))
                } else {
                    Err("> can only act on a pair of numbers")
                }
            }
            TokenType::LessEqual => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Bool(v_left <= v_right))
                } else {
                    Err("<= can only act on a pair of numbers")
                }
            }
            TokenType::GreaterEqual => {
                if let (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) = (left, right)
                {
                    Ok(LiteralValue::Bool(v_left >= v_right))
                } else {
                    Err(">= can only act on a pair of numbers")
                }
            }
            TokenType::BangEqual => Ok(LiteralValue::Bool(!(left == right))),
            TokenType::EqualEqual => Ok(LiteralValue::Bool(left == right)),
            TokenType::Plus => match (left, right) {
                (LiteralValue::Number(v_left), LiteralValue::Number(v_right)) => {
                    Ok(LiteralValue::Number(v_left + v_right))
                }
                (LiteralValue::String(v_left), LiteralValue::String(v_right)) => {
                    Ok(LiteralValue::String(format!("{}{}", v_left, v_right)))
                }
                _ => Err("addition can only act on a pair of numbers or strings"),
            },
            _ => Err("binary operation can only have operator  '-', '+', '*', '/', '<', '>', '<=', '>=','==', '!='"),
        }
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<LiteralValue, &'static str> {
        Ok(expr.value.clone())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<LiteralValue, &'static str> {
        self.evaluate(&expr.expr)
    }
}

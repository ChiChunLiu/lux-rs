use std::fmt;

use crate::token::Token;

pub trait Accept<R> {
    fn accept(&self, visitor: &impl ExprVisitor<R>) -> R;
}

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> R;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> R;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> R;
}

#[macro_export]
macro_rules! ast_node {
    ( $node_name:ident,  $(($field_name:ident, $field_type:ident)),* ) => {
        #[derive(Clone, Debug)]
        pub struct $node_name {
            $(
                pub $field_name: $field_type,
            )*
        }

        paste::paste! {
        impl<'a, R> Accept<R> for $node_name {
           fn accept(&self, visitor: &impl ExprVisitor<R>) -> R {
               visitor.[<visit_ $node_name:snake>](self)
           }
        }
        }
    };
}

#[derive(Clone, PartialEq, Debug)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Bool(value) => format!("Bool({})", value),
            Self::Number(value) => format!("Number({})", value),
            Self::String(value) => format!("String({})", value),
            Self::Nil => "Nil".to_string(),
        };
        write!(f, "{}", message)
    }
}

ast_node!(BinaryExpr, (left, Expr), (operator, Token), (right, Expr));
ast_node!(UnaryExpr, (operator, Token), (right, Expr));
ast_node!(LiteralExpr, (value, LiteralValue));
ast_node!(GroupingExpr, (expr, Expr));

// Box is necessary because expression created inside a function
// needs to be owned
#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Literal(Box<LiteralExpr>),
    Grouping(Box<GroupingExpr>),
}

impl<R> Accept<R> for Expr {
    fn accept(&self, visitor: &impl ExprVisitor<R>) -> R {
        match self {
            Self::Binary(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Literal(expr) => expr.accept(visitor),
            Self::Grouping(expr) => expr.accept(visitor),
        }
    }
}

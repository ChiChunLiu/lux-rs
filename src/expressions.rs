use crate::token::Token;
use paste;

trait Accept<R> {
    fn accept(&self, visitor: &impl ExprVisitor<R>) -> R;
}

trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> R;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> R;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> R;
}

#[macro_export]
macro_rules! ast_node {
    ( $node_name:ident,  $(($field_name:ident, $field_type:ident)),* ) => {
        pub struct $node_name<'a> {
            $(
                pub $field_name: $field_type<'a>,
            )*
        }

        paste::paste! {
        impl<'a, R> Accept<R> for $node_name<'a> {
           fn accept(&self, visitor: &impl ExprVisitor<R>) -> R {
               visitor.[<visit_ $node_name:snake>](self)
           }
        }
        }
    };
}

pub enum LiteralValue<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Nil,
}

ast_node!(BinaryExpr, (left, Expr), (operator, Token), (right, Expr));
ast_node!(UnaryExpr, (operator, Token), (right, Expr));
ast_node!(LiteralExpr, (value, LiteralValue));
ast_node!(GroupingExpr, (expr, Expr));

// Box is necessary because expression created inside a function
// needs to be owned
pub enum Expr<'a> {
    Binary(Box<BinaryExpr<'a>>),
    Unary(Box<UnaryExpr<'a>>),
    Literal(Box<LiteralExpr<'a>>),
    Grouping(Box<GroupingExpr<'a>>),
}

impl<'a, R> Accept<R> for Expr<'a> {
    fn accept(&self, visitor: &impl ExprVisitor<R>) -> R {
        match self {
            Self::Binary(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Literal(expr) => expr.accept(visitor),
            Self::Grouping(expr) => expr.accept(visitor),
        }
    }
}

struct AstPrinter;
impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(&format!("{}", name));
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self))
        }
        result.push(')');
        result
    }
}
impl ExprVisitor<String> for AstPrinter {
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme, &[&expr.right])
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match expr.value {
            LiteralValue::Number(v) => format!("{}", v),
            LiteralValue::String(v) => v.to_owned(),
            LiteralValue::Bool(v) => format!("{}", v),
            LiteralValue::Nil => String::from("nil"),
        }
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", &[&expr.expr])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_ast_printer() {
        let expression = Expr::Binary(Box::new(BinaryExpr {
            left: Expr::Unary(Box::new(UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-",
                    line: 1,
                },
                right: Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Number(123.0),
                })),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*",
                line: 1,
            },
            right: Expr::Grouping(Box::new(GroupingExpr {
                expr: Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::String("abc"),
                })),
            })),
        }));
        let visitor = AstPrinter {};
        let printed = expression.accept(&visitor);
        assert_eq!(printed, "(* (- 123) (group abc))")
    }
}

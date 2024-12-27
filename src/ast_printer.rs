use crate::expressions::{
    Accept, BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, LiteralValue, UnaryExpr,
};

pub struct AstPrinter;
impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
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
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match &expr.value {
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
    use crate::token::{Token, TokenType};

    #[test]
    fn test_ast_printer() {
        let expression = Expr::Binary(Box::new(BinaryExpr {
            left: Expr::Unary(Box::new(UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    line: 1,
                },
                right: Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::Number(123.0),
                })),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                line: 1,
            },
            right: Expr::Grouping(Box::new(GroupingExpr {
                expr: Expr::Literal(Box::new(LiteralExpr {
                    value: LiteralValue::String("abc".to_string()),
                })),
            })),
        }));
        let visitor = AstPrinter {};
        let printed = expression.accept(&visitor);
        assert_eq!(printed, "(* (- 123) (group abc))")
    }
}

use crate::expressions::Expr;

pub trait Accept<R> {
    fn accept(&self, visitor: &impl StmtVisitor<R>) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> R;
    fn visit_expr_stmt(&self, stmt: &ExprStmt) -> R;
}

#[macro_export]
macro_rules! stmt {
    ( $node_name:ident,  $(($field_name:ident, $field_type:ident)),* ) => {
        #[derive(Clone, Debug)]
        pub struct $node_name {
            $(
                pub $field_name: $field_type,
            )*
        }

        paste::paste! {
        impl<'a, R> Accept<R> for $node_name {
           fn accept(&self, visitor: &impl StmtVisitor<R>) -> R {
               visitor.[<visit_ $node_name:snake>](self)
           }
        }
        }
    };
}

stmt!(PrintStmt, (expr, Expr));
stmt!(ExprStmt, (expr, Expr));

// Box is necessary because expression created inside a function
// needs to be owned
#[derive(Clone, Debug)]
pub enum Stmt {
    Print(Box<PrintStmt>),
    Expr(Box<ExprStmt>),
}

impl<R> Accept<R> for Stmt {
    fn accept(&self, visitor: &impl StmtVisitor<R>) -> R {
        match self {
            Self::Print(stmt) => stmt.accept(visitor),
            Self::Expr(stmt) => stmt.accept(visitor),
        }
    }
}

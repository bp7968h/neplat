use super::{expr::Expr, stmt::Stmt};

pub trait ExprVisitor<T> {
    fn visit_binary_expression(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expression(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
}
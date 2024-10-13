use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_binary_expression(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expression(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}
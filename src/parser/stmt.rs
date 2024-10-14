use super::{expr::Expr, visitor::StmtVisitor};



#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression_stmt(self),
            Stmt::Print(expr) => visitor.visit_print_stmt(self),
        }
    }
}
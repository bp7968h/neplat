use crate::lexer::Token;

use super::{expr::Expr, visitor::StmtVisitor};



#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>)
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(_expr) => visitor.visit_expression_stmt(self),
            Stmt::Print(_expr) => visitor.visit_print_stmt(self),
            Stmt::Var(_token, _expr) => visitor.visit_var_stmt(self),
        }
    }
}
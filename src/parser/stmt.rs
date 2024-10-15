use core::fmt;

use crate::lexer::Token;

use super::{expr::Expr, visitor::StmtVisitor};



#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    //condition, then, else 
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    // condition, body
    While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Block(_stmt_list) => visitor.visit_block_stmt(self),
            Stmt::Expression(_expr) => visitor.visit_expression_stmt(self),
            Stmt::Print(_expr) => visitor.visit_print_stmt(self),
            Stmt::Var(_token, _expr) => visitor.visit_var_stmt(self),
            Stmt::If(_expr, _stmt_then, _stmt_else) => visitor.visit_if_stmt(self),
            Stmt::While(_expr, _stmt) => visitor.visit_while_stmt(self),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(list) => {
                for stmt in list {
                    write!(f, "{} ", stmt)?
                }

                Ok(())
            },
            Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "{}", expr),
            Stmt::Var(tok,expr ) => write!(f, "{} {:?}", tok, expr),
            Stmt::If(condition, then_stmt, else_stmt ) => write!(f, "{} {} {:?}", condition, then_stmt, else_stmt),
            Stmt::While(condition, body) => write!(f, "{} {}", condition, *body)
        }
    }
}
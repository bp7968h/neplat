use core::fmt;

use crate::lexer::{Literal, Token};

use super::visitor::ExprVisitor;

#[derive(Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Assign(_token, _expr) => visitor.visit_assign_expression(self),
            Expr::Binary(_left, _operator, _right) => visitor.visit_binary_expression(self),
            Expr::Grouping(_group) => visitor.visit_grouping_expression(self),
            Expr::Literal(_value) => visitor.visit_literal_expr(self),
            Expr::Unary(_operator, _operand) => visitor.visit_unary_expr(self),
            Expr::Variable(_token) => visitor.vist_variable_expr(self),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Display assign expression in format
            Expr::Assign(token, expr) => {
                write!(f, "assigned {} to {}", expr, token.lexeme())
            }

            // Display binary expressions in the format "(left operator right)"
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", left, operator.lexeme(), right)
            }
            
            // Display unary expressions in the format "(operator operand)"
            Expr::Unary(operator, operand) => {
                write!(f, "({} {})", operator.lexeme(), operand)
            }

            // Display literals as their actual values
            Expr::Literal(literal) => {
                match literal {
                    Literal::BooleanLiteral(b) => write!(f, "{}", b),
                    Literal::NumberLiteral(n) => write!(f, "{}", n),
                    Literal::StringLiteral(s) => write!(f, "\"{}\"", s),
                    Literal::NullLiteral => write!(f, "null"),
                }
            }

            // Display grouped expressions in parentheses
            Expr::Grouping(expression) => {
                write!(f, "(group {})", expression)
            }

            Expr::Variable(token) => {
                write!(f, "(varibale {:?})", token)
            }
        }
    }
}
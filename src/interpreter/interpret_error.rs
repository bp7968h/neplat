use core::fmt;

use crate::lexer::TokenType;

#[derive(Debug)]
pub enum InterpretError {
    DivisionByZero,
    TypeMismatch(String),
    UnsupportedOperator(TokenType),
    UnexpectedError(String),
    UndefinedVariable(String),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::DivisionByZero => write!(f, "Error: Cannot divide by zero"),
            InterpretError::TypeMismatch(err_str) => write!(f, "Error: {}", err_str),
            InterpretError::UnexpectedError(err_str) => write!(f, "Error: {}", err_str),
            InterpretError::UnsupportedOperator(token_type) => write!(f, "Error: Unknown type {:?}", token_type),
            InterpretError::UndefinedVariable(err_str) => write!(f, "Error: Undefined variable '{}'", err_str),
        }
    }
}
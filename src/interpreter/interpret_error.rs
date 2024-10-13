use crate::lexer::TokenType;

pub enum InterpretError {
    DivisionByZero,
    TypeMismatch(String),
    UnsupportedOperator(TokenType),
}
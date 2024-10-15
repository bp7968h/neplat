use std::{fmt, rc::Rc};

use crate::interpreter::callable::Callable;

use super::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        let lexeme = lexeme.to_string();
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn literal(&self) -> &Option<Literal> {
        &self.literal
    }

    pub fn line(&self) -> &usize {
        &self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)?;
        match &self.literal {
            Some(literal) => write!(f, " {}", literal)?,
            None => write!(f, "")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
    Callable(Rc<dyn Callable>),
}

impl Literal {
    pub fn as_callable(&self) -> Option<&dyn Callable> {
        if let Literal::Callable(ref callable) = self {
            Some(callable.as_ref())
        } else {
            None
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match  self {
            Self::BooleanLiteral(bool) => write!(f, "{}",bool),
            Self::NullLiteral => write!(f, "Null"),
            Self::NumberLiteral(num) => write!(f, "{}", num),
            Self::StringLiteral(str) => write!(f, "{}", str),
            Self::Callable(c) => write!(f, "{:?}", c),
        }
    }
}

use std::fmt;

use super::TokenType;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match  self {
            Self::BooleanLiteral(bool) => write!(f, "{}",bool),
            Self::NullLiteral => write!(f, "Null"),
            Self::NumberLiteral(num) => write!(f, "{}", num),
            Self::StringLiteral(str) => write!(f, "{}", str),
        }
    }
}

use core::fmt;

#[derive(Debug)]
pub enum ParserError {
    UnclosedParen {
        line: usize,
        lexeme: String,
    },
    ExpectedExpression {
        line: usize,
        lexeme: String,
    },
    InvalidLiteral {
        line: usize,
        lexeme: String,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnclosedParen { line, lexeme } => {
                if lexeme.is_empty() {
                    write!(f, "{} at end: Expect ')' after expression.", line)
                } else {
                    write!(f, "{} at '{}': Expect ')' after expression.", line, lexeme)
                }
            },
            ParserError::ExpectedExpression { line, lexeme } => {
                write!(f, "{} at '{}': Expected expression.", line, lexeme)
            },
            ParserError::InvalidLiteral { line, lexeme } => {
                write!(f, "{} at '{}': Invalid literal.", line, lexeme)
            },
        }
    }
}
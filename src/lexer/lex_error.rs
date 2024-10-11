use core::fmt;

#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter(char, usize),
    UnterminatedString(usize),
    InvalidNumber(usize),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter(char, line) => write!(f, "Err: Unexpected character '{}' at line {}", char, line),
            Self::UnterminatedString(line) => write!(f, "Err: Unterminated string at line {}", line),
            Self::InvalidNumber(line) => write!(f, "Err: Invalid number at line {}", line),
        }
    }
}
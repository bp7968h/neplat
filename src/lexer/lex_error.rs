#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter(char, usize),
    UnterminatedString(usize),
    InvalidNumber(usize),
}
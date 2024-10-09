pub struct Lexer {
    source: String,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            line: 1
        }
    }
}
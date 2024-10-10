use super::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(super::TokenType::EOF, "", None, self.line));
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let single_character = self.advance();

        match single_character {
                b'(' => Some(self.create_token(TokenType::LEFTPAREN)),
                b')' => Some(self.create_token(TokenType::RIGHTPAREN)),
                b'{' => Some(self.create_token(TokenType::LEFTBRACE)),
                b'}' => Some(self.create_token(TokenType::RIGHTBRACE)),
                b',' => Some(self.create_token(TokenType::COMMA)),
                b'.' => Some(self.create_token(TokenType::DOT)),
                b'-' => Some(self.create_token(TokenType::MINUS)),
                b'+' => Some(self.create_token(TokenType::PLUS)),
                b';' => Some(self.create_token(TokenType::SEMICOLON)),
                b'*' => Some(self.create_token(TokenType::STAR)),
                b'!' => {
                    if self.match_char('=') {
                        Some(self.create_token(TokenType::BANGEQUAL))
                    } else {
                        Some(self.create_token(TokenType::BANG))
                    }
                },
                _ => todo!(),
            }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        if (self.source[self.current] as char) != expected {
            return false
        }

        self.current += 1;
        true
    }

    fn create_token(&self, token: TokenType) -> Token {
        Token::new(
            token,
            "",
            None, 
            self.line
        )
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

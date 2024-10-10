use super::{Token, TokenType};

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
            Some(c) => match c {
                '(' => Some(self.create_token(TokenType::LEFTPAREN)),
                ')' => Some(self.create_token(TokenType::RIGHTPAREN)),
                '{' => Some(self.create_token(TokenType::LEFTBRACE)),
                '}' => Some(self.create_token(TokenType::RIGHTBRACE)),
                ',' => Some(self.create_token(TokenType::COMMA)),
                '.' => Some(self.create_token(TokenType::DOT)),
                '-' => Some(self.create_token(TokenType::MINUS)),
                '+' => Some(self.create_token(TokenType::PLUS)),
                ';' => Some(self.create_token(TokenType::SEMICOLON)),
                '*' => Some(self.create_token(TokenType::STAR)),
                '!' => {
                    if self.match_char('=') {
                        Some(self.create_token(TokenType::BANGEQUAL))
                    } else {
                        Some(self.create_token(TokenType::BANG))
                    }
                },
                _ => todo!(),
            },
            None => {
                todo!()
            }
        }
    }

    fn match_char(&self, c: char) -> bool {
        if (self.is_at_end()) {
            return false
        }

        // if let Some()

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

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let c = self.source.chars().nth(self.current);
            self.current += 1;
            c
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

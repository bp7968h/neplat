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
                        return Some(self.create_token(TokenType::BANGEQUAL))
                    }
                    Some(self.create_token(TokenType::BANG))
                },
                b'=' => {
                    if self.match_char('=') {
                        return Some(self.create_token(TokenType::EQUALEQUAL))
                    }
                    Some(self.create_token(TokenType::EQUAL))
                },
                b'<' => {
                    if self.match_char('=') {
                        return Some(self.create_token(TokenType::LESSEQUAL))
                    }
                    Some(self.create_token(TokenType::LESS))
                },
                b'>' => {
                    if self.match_char('=') {
                        return Some(self.create_token(TokenType::GREATEREQUAL))
                    }
                    Some(self.create_token(TokenType::GREATER))
                },
                b'/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        None
                    } else if self.match_char('*') {
                        while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                            if self.peek() == '\n' {
                                self.line += 1;
                            }
                            self.advance();
                        }

                        if !self.is_at_end() {
                            self.advance();
                        }

                        if !self.is_at_end() {
                            self.advance();
                        }

                        None
                    } else {
                        Some(self.create_token(TokenType::SLASH))
                    }
                },
                b' ' | b'\r' | b'\t' => None,
                b'\n' => {
                    self.line += 1;
                    None
                },
                b'"' => {
                    todo!()
                },
                other => {
                    todo!()
                }
            }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1] as char 
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current] as char
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

use super::{LexError, Literal, Token, TokenType};

pub struct Lexer<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
    errors: Vec<LexError>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<LexError> {
        &self.errors
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
                    return Some(self.create_token(TokenType::BANGEQUAL));
                }
                Some(self.create_token(TokenType::BANG))
            }
            b'=' => {
                if self.match_char('=') {
                    return Some(self.create_token(TokenType::EQUALEQUAL));
                }
                Some(self.create_token(TokenType::EQUAL))
            }
            b'<' => {
                if self.match_char('=') {
                    return Some(self.create_token(TokenType::LESSEQUAL));
                }
                Some(self.create_token(TokenType::LESS))
            }
            b'>' => {
                if self.match_char('=') {
                    return Some(self.create_token(TokenType::GREATEREQUAL));
                }
                Some(self.create_token(TokenType::GREATER))
            }
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
            }
            b' ' | b'\r' | b'\t' => None,
            b'\n' => {
                self.line += 1;
                None
            }
            b'"' => Some(self.handle_string()),
            c @ b'0'..=b'9' => {
                Some(self.handle_number(c))
            },
            c @ b'a'..=b'z' | c @ b'A'..=b'Z' | c @ b'_' => Some(self.handle_identifier(c as char)),
            other => {
                self.errors.push(LexError::UnexpectedCharacter(other as char, self.line));
                None
            },
        }
    }

    fn handle_identifier(&mut self, curr_char: char) -> Token {
        let mut value = String::from(curr_char);

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            value.push(self.advance() as char);
        }

        let token_type = match value.as_str() {
            "true" | "satya" => TokenType::TRUE,
            "false" | "galat" => TokenType::FALSE,
            "and" | "ra" => TokenType::AND,
            "or" | "wa" => TokenType::OR,
            "if" | "yadi" => TokenType::IF,
            "else" | "athwa" => TokenType::ELSE,
            "func" | "karya" => TokenType::FUNC,
            "return" | "dinus" => TokenType::RETURN,
            "for" | "ko_lagi" => TokenType::FOR,
            "null" | "khali" => TokenType::NULL,
            "print" | "dekhau" => TokenType::PRINT,
            "let" | "manum" => TokenType::VAR,
            "while" | "jaba_samma" => TokenType::WHILE,
            "class" | "samuha" => TokenType::CLASS,
            "this" | "yei" => TokenType::THIS,
            "super" | "affnai" => TokenType::SUPER,
            "ghatau" => TokenType::MINUS,
            "joda" => TokenType::PLUS,
            "ulto" => TokenType::BANG,
            "barabar_chaina" => TokenType::BANGEQUAL,
            "bhaneko" => TokenType::EQUAL,
            "barabar" => TokenType::EQUALEQUAL,
            "bhanda_thulo" => TokenType::GREATER,
            "thulo_wa_barabar" => TokenType::GREATEREQUAL,
            "bhanda_sano" => TokenType::LESS,
            "sano_wa_barabar" => TokenType::LESSEQUAL,
            _ => TokenType::IDENTIFIER,
        };

        Token::new(token_type, &value, None, self.line)
    }

    fn handle_number(&mut self, curr_num: u8) -> Token {
        let mut value = String::from(curr_num as char);

        while self.peek().is_ascii_digit() {
            value.push(self.advance() as char);
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance() as char);

            while self.peek().is_ascii_digit() {
                value.push(self.advance() as char);
            }
        }

        match value.parse::<f64>() {
            Ok(number_value) => Token::new(
                TokenType::NUMBER,
                &value,
                Some(Literal::NumberLiteral(number_value)),
                self.line,
            ),
            Err(_) => {
                self.errors.push(LexError::InvalidNumber(self.line));
                Token::new(TokenType::NUMBER, &value, None, self.line)
            }
        }
    }

    fn handle_string(&mut self) -> Token {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            value.push(self.advance() as char);
        }

        if self.is_at_end() {
            self.errors.push(LexError::UnterminatedString(self.line));
            return Token::new(TokenType::STRING, &value, None, self.line);
        }

        self.advance();

        Token::new(TokenType::STRING, &value, None, self.line)
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
            return false;
        }

        if (self.source[self.current] as char) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn create_token(&self, token: TokenType) -> Token {
        Token::new(token, "", None, self.line)
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

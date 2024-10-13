use crate::lexer::{Literal, Token, TokenType};

use super::{expr::Expr, parser_error::ParserError};

pub struct Parser<'a> {
    current: usize,
    tokens: &'a [Token],
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let expr = self.expression();

        if self.errors.is_empty() {
            expr
        } else {
            None
        }
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Some(expr)
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Some(expr)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_token_types(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Some(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_token_types(&[TokenType::FALSE]) {
            return Some(Expr::Literal(Literal::BooleanLiteral(false)));
        }

        if self.match_token_types(&[TokenType::TRUE]) {
            return Some(Expr::Literal(Literal::BooleanLiteral(true)));
        }

        if self.match_token_types(&[TokenType::NULL]) {
            return Some(Expr::Literal(Literal::NullLiteral));
        }

        if self.match_token_types(&[TokenType::NUMBER, TokenType::STRING]) {
            if let Some(literal) = self.previous().literal() {
                return Some(Expr::Literal(literal.clone()));
            } else {
                self.errors.push(ParserError::InvalidLiteral {
                    line: self.previous().line().clone(),
                    lexeme: self.previous().lexeme().to_string(),
                });
                return None;
            }
        }

        if self.match_token_types(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;

            if self.consume(&TokenType::RIGHTPAREN).is_none() {
                return None;
            }

            return Some(Expr::Grouping(Box::new(expr)));
        }

        self.errors.push(ParserError::ExpectedExpression {
            line: self.peek().line().clone(),
            lexeme: self.peek().lexeme().to_string(),
        });

        None
    }

    fn consume(&mut self, token_type: &TokenType) -> Option<&Token> {
        if self.check(token_type) {
            return Some(self.advance());
        }

        let peeked_token = self.peek();

        if self.is_at_end() {
            self.errors.push(ParserError::UnclosedParen {
                line: peeked_token.line().clone(),
                lexeme: String::from("end"),
            });
        } else {
            self.errors.push(ParserError::UnclosedParen {
                line: peeked_token.line().clone(),
                lexeme: peeked_token.lexeme().to_string(),
            });
        }

        None
    }

    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == &TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type() {
                &TokenType::FUNC
                | &TokenType::CLASS
                | &TokenType::VAR
                | &TokenType::FOR
                | &TokenType::WHILE
                | &TokenType::IF
                | &TokenType::PRINT
                | &TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn match_token_types(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        // println!(
        //     "advance() -> current: {}, previous: {:?}",
        //     self.current,
        //     self.previous()
        // );
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == &TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

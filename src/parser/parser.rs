use crate::lexer::{Literal, Token, TokenType};

use super::{expr::Expr, parser_error::ParserError, stmt::Stmt};

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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.errors.is_empty() {
            if self.match_token_types(&[TokenType::VAR]) {
                return self.var_declaration();
            }

            return self.statement();
        }

        self.synchronize();
        None
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token_types(&[TokenType::IF]) {
            return self.if_statement();
        }

        if self.match_token_types(&[TokenType::PRINT]) {
            return self.print_statement();
        }

        if self.match_token_types(&[TokenType::LEFTBRACE]) {
            return Some(Stmt::Block(self.block()));
        }

        if self.match_token_types(&[TokenType::WHILE]) {
            return self.while_statement();
        }

        if self.match_token_types(&[TokenType::FOR]) {
            return self.for_statement();
        }

        if self.match_token_types(&[TokenType::FUNC]) {
            return self.function_statement("function");
        }

        self.expression_statement()
    }

    fn function_statement(&mut self, kind: &str) -> Option<Stmt> {
        let name = self.consume(&TokenType::IDENTIFIER).cloned();
        if name.is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: format!("Expected {} name.", kind),
            });
            return None;
        }

        if self.consume(&TokenType::LEFTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: format!("Expected '(' after {} name.", kind),
            });
            return None;
        }

        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if parameters.len() >= 255 {
                    self.errors.push(ParserError::MaxFunctionArguments {
                        line: self.peek().line().clone(),
                        lexeme: "Cannot have more than 255 arguments.".to_string(),
                    });
                    return None;
                }

                if let Some(parameter) = self.consume(&TokenType::IDENTIFIER).cloned() {
                    parameters.push(parameter);
                } else {
                    return None;
                }

                if !self.match_token_types(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        if self.consume(&TokenType::RIGHTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expected ')' after parameters".to_string(),
            });
            return None;
        }

        if self.consume(&TokenType::LEFTBRACE).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: format!("Expect '{{' before {} body.", kind),
            });
            return None;
        }

        let body = self.block();
        
        Some(Stmt::Function(name.unwrap(), parameters, body))

    }

    fn for_statement(&mut self) -> Option<Stmt> {
        if self.consume(&TokenType::LEFTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect '(' after 'for'.".to_string(),
            });
            return None;
        }

        let initializer = if self.match_token_types(&[TokenType::SEMICOLON]) {
            None
        } else if self.match_token_types(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.expression_statement()
        };

        let condition = if !self.check(&TokenType::SEMICOLON) {
            self.expression()
        } else {
            None
        };

        if self.consume(&TokenType::SEMICOLON).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect ';' after loop condition.".to_string(),
            });
            return None;
        }

        let increment = if !self.check(&TokenType::RIGHTPAREN) {
            self.expression()
        } else {
            None
        };

        if self.consume(&TokenType::RIGHTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect ')' after for clauses.".to_string(),
            });
            return None;
        }

        let mut body = self.statement()?;

        // If increment is present, execute it after the body
        if let Some(increment) = increment {
            body = Stmt::Block(vec![Box::new(body), Box::new(Stmt::Expression(increment))]);
        }

        // If no condition is present, assume `true` (infinite loop)
        let condition = condition.unwrap_or(Expr::Literal(Literal::BooleanLiteral(true)));
        // Wrap the body in a while loop using the condition
        body = Stmt::While(condition, Box::new(body));

        // If initializer exists, execute it before the loop
        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![Box::new(initializer), Box::new(body)]);
        }

        Some(body)
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        if self.consume(&TokenType::LEFTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect '(' after 'while'.".to_string(),
            });
            return None;
        }

        let condition = self.expression()?;

        if self.consume(&TokenType::RIGHTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect ')' after 'while' condition.".to_string(),
            });
            return None;
        }

        let body = Box::new(self.statement()?);

        Some(Stmt::While(condition, body))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        if self.consume(&TokenType::LEFTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect '(' after 'if'.".to_string(),
            });
            return None;
        }

        let condition = self.expression()?;

        if self.consume(&TokenType::RIGHTPAREN).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expect ')' after 'if' condition.".to_string(),
            });
            return None;
        }

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token_types(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Some(Stmt::If(condition, then_branch, else_branch))
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression()?;

        if self.consume(&TokenType::SEMICOLON).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expected ';' after value.".to_string(),
            });
            return None;
        }

        Some(Stmt::Print(expr))
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        if let Some(var_name) = self.consume(&TokenType::IDENTIFIER).cloned() {
            let mut initializer: Option<Expr> = None;
            if self.match_token_types(&[TokenType::EQUAL]) {
                initializer = self.expression();
            }

            if self.consume(&TokenType::SEMICOLON).is_none() {
                self.errors.push(ParserError::ExpectedExpression {
                    line: self.peek().line().clone(),
                    lexeme: "Expected ';' after value.".to_string(),
                });
                return None;
            }

            return Some(Stmt::Var(var_name, initializer));
        }

        self.errors.push(ParserError::InvalidDecleration {
            line: self.peek().line().clone(),
            lexeme: "Expect variable name.".to_string(),
        });
        None
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression()?;

        if self.consume(&TokenType::SEMICOLON).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expected ';' after expression.".to_string(),
            });
            return None;
        }

        Some(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Vec<Box<Stmt>> {
        let mut statements: Vec<Box<Stmt>> = Vec::new();

        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(Box::new(stmt));
            }
        }

        if self.consume(&TokenType::RIGHTBRACE).is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expected '}' after block.".to_string(),
            });
        }

        return statements;
    }

    fn expression(&mut self) -> Option<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        let expr = self.or()?;

        if self.match_token_types(&[TokenType::EQUAL]) {
            let equals = self.previous().clone();

            if let Some(value) = self.assignment() {
                if let Expr::Variable(name) = expr {
                    return Some(Expr::Assign(name, Box::new(value)));
                } else {
                    self.errors.push(ParserError::InvalidAssignment {
                        line: equals.line().clone(),
                        lexeme: equals.lexeme().to_string(),
                    });
                }
            }
        }

        Some(expr)
    }

    fn or(&mut self) -> Option<Expr> {
        let mut expr = self.and()?;

        while self.match_token_types(&[TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Some(expr)
    }

    fn and(&mut self) -> Option<Expr> {
        let mut expr = self.equality()?;

        while self.match_token_types(&[TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Some(expr)
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

        self.call()
    }

    fn call(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token_types(&[TokenType::LEFTPAREN]) {
                if let Some(exp) = self.finish_call(expr.clone()) {
                    expr = exp;
                }
            } else {
                break;
            }
        }

        Some(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();

        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    self.errors.push(ParserError::MaxFunctionArguments {
                        line: self.peek().line().clone(),
                        lexeme: "Cannot have more than 255 arguments.".to_string(),
                    });
                    return None;
                }

                if let Some(expr) = self.expression() {
                    arguments.push(Box::new(expr));
                }

                if !self.match_token_types(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RIGHTPAREN);
        if paren.is_none() {
            self.errors.push(ParserError::ExpectedExpression {
                line: self.peek().line().clone(),
                lexeme: "Expected ')' after arguments.".to_string(),
            });
            return None;
        }

        Some(Expr::Call(
            Box::new(callee),
            paren.unwrap().clone(),
            arguments,
        ))
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

        if self.match_token_types(&[TokenType::IDENTIFIER]) {
            return Some(Expr::Variable(self.previous().clone()));
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

    fn synchronize(&mut self) {
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

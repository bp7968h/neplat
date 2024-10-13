use neplat::{
    lexer::{Literal, Token, TokenType},
    parser::{expr::Expr, parser_error::ParserError},
    Parser,
};

#[test]
fn test_parse_simple_binary_expression() {
    // Tokens: 1 + 2
    let tokens = vec![
        Token::new(TokenType::NUMBER, "1", Some(Literal::NumberLiteral(1.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "2", Some(Literal::NumberLiteral(2.0)), 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();
    assert!(matches!(ast, Some(Expr::Binary(..))));

    if let Some(Expr::Binary(left, operator, right)) = ast {
        assert!(matches!(*left, Expr::Literal(Literal::NumberLiteral(1.0))));
        assert_eq!(operator.token_type(), &TokenType::PLUS);
        assert!(matches!(*right, Expr::Literal(Literal::NumberLiteral(2.0))));
    }
}

#[test]
fn test_parse_binary_expression_with_precedence() {
    // Tokens: 1 + 2 * 3
    let tokens = vec![
        Token::new(TokenType::NUMBER, "1", Some(Literal::NumberLiteral(1.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "2", Some(Literal::NumberLiteral(2.0)), 1),
        Token::new(TokenType::STAR, "*", None, 1),
        Token::new(TokenType::NUMBER, "3", Some(Literal::NumberLiteral(3.0)), 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    // Expected AST: (1 + (2 * 3))
    assert!(matches!(ast, Some(Expr::Binary(..))));
    if let Some(Expr::Binary(left, operator, right)) = ast {
        assert!(matches!(*left, Expr::Literal(Literal::NumberLiteral(1.0))));
        assert_eq!(operator.token_type(), &TokenType::PLUS);

        // Right part should be a multiplication
        if let Expr::Binary(inner_left, inner_operator, inner_right) = *right {
            assert!(matches!(
                *inner_left,
                Expr::Literal(Literal::NumberLiteral(2.0))
            ));
            assert_eq!(inner_operator.token_type(), &TokenType::STAR);
            assert!(matches!(
                *inner_right,
                Expr::Literal(Literal::NumberLiteral(3.0))
            ));
        } else {
            panic!("Expected multiplication on the right side");
        }
    }
}

// Test case for parsing unary expressions
#[test]
fn test_parse_unary_expression() {
    // Tokens: -5
    let tokens = vec![
        Token::new(TokenType::MINUS, "-", None, 1),
        Token::new(TokenType::NUMBER, "5", Some(Literal::NumberLiteral(5.0)), 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    // Expected AST: (-5)
    assert!(matches!(ast, Some(Expr::Unary(..))));
    if let Some(Expr::Unary(operator, operand)) = ast {
        assert_eq!(operator.token_type(), &TokenType::MINUS);
        assert!(matches!(
            *operand,
            Expr::Literal(Literal::NumberLiteral(5.0))
        ));
    }
}

// Test case for parsing grouping expressions
#[test]
fn test_parse_grouping_expression() {
    // Tokens: (1 + 2)
    let tokens = vec![
        Token::new(TokenType::LEFTPAREN, "(", None, 1),
        Token::new(TokenType::NUMBER, "1", Some(Literal::NumberLiteral(1.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "2", Some(Literal::NumberLiteral(2.0)), 1),
        Token::new(TokenType::RIGHTPAREN, ")", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    // Expected AST: Grouping(1 + 2)
    assert!(matches!(ast, Some(Expr::Grouping(..))));
    if let Some(Expr::Grouping(inner_expr)) = ast {
        if let Expr::Binary(left, operator, right) = *inner_expr {
            assert!(matches!(*left, Expr::Literal(Literal::NumberLiteral(1.0))));
            assert_eq!(operator.token_type(), &TokenType::PLUS);
            assert!(matches!(*right, Expr::Literal(Literal::NumberLiteral(2.0))));
        }
    }
}

// Test case for error handling (unclosed parenthesis)
#[test]
fn test_parse_unclosed_parenthesis_error() {
    // Tokens: (1 + 2
    let tokens = vec![
        Token::new(TokenType::LEFTPAREN, "(", None, 1),
        Token::new(TokenType::NUMBER, "1", Some(Literal::NumberLiteral(1.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "2", Some(Literal::NumberLiteral(2.0)), 1),
        // Missing closing parenthesis
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    // Should return None and an error for unclosed parenthesis
    assert!(ast.is_none());
    let errors = parser.get_errors();

    assert_eq!(errors.len(), 1);
    assert!(matches!(
        errors[0],
        ParserError::UnclosedParen { line: 1, .. }
    ));
}

use neplat::{lexer::{Literal, Token, TokenType}, parser::stmt::Stmt, Interpreter, Parser};

#[test]
fn test_interpreter_simple_addition() {
    // Tokens for the expression: 1 + 2;
    let tokens = vec![
        Token::new(TokenType::NUMBER, "1", Some(Literal::NumberLiteral(1.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "2", Some(Literal::NumberLiteral(2.0)), 1),
        Token::new(TokenType::SEMICOLON, ";", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    assert_eq!(ast.len(), 1);
    assert!(matches!(ast[0], Stmt::Expression(_)));

    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);

    assert!(interpreter.get_errors().is_empty());
}

#[test]
fn test_interpreter_print_statement() {
    // Tokens for the statement: print 3 + 4;
    let tokens = vec![
        Token::new(TokenType::PRINT, "print", None, 1),
        Token::new(TokenType::NUMBER, "3", Some(Literal::NumberLiteral(3.0)), 1),
        Token::new(TokenType::PLUS, "+", None, 1),
        Token::new(TokenType::NUMBER, "4", Some(Literal::NumberLiteral(4.0)), 1),
        Token::new(TokenType::SEMICOLON, ";", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);

    //TODO capture the stdout and validate the output

    assert!(interpreter.get_errors().is_empty());
}

#[test]
fn test_interpreter_variable_declaration() {
    // Tokens for the statement: var x = 5;
    let tokens = vec![
        Token::new(TokenType::VAR, "var", None, 1),
        Token::new(TokenType::IDENTIFIER, "x", None, 1),
        Token::new(TokenType::EQUAL, "=", None, 1),
        Token::new(TokenType::NUMBER, "5", Some(Literal::NumberLiteral(5.0)), 1),
        Token::new(TokenType::SEMICOLON, ";", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ];

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);


    assert!(interpreter.get_errors().is_empty());

    let value = interpreter.get_variable("x");
    assert!(matches!(value, Some(Literal::NumberLiteral(5.0))));
}

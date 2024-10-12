use neplat::lexer::{LexError, Lexer, TokenType};

#[test]
fn test_single_number() {
    let mut lexer = Lexer::new("123".as_bytes());
    let tokens = lexer.tokenize();

    // One number token + EOF token
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), &TokenType::NUMBER);
    assert_eq!(tokens[0].lexeme(), "123");
}

#[test]
fn test_identifier() {
    let mut lexer = Lexer::new("foo".as_bytes());
    let tokens = lexer.tokenize();

    // One identifier token + EOF token
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), &TokenType::IDENTIFIER);
    assert_eq!(tokens[0].lexeme(), "foo");
}

#[test]
fn test_operators() {
    let mut lexer = Lexer::new("+ - * /".as_bytes());
    let tokens = lexer.tokenize();

    // Four operators + EOF token
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type(), &TokenType::PLUS);
    assert_eq!(tokens[1].token_type(), &TokenType::MINUS);
    assert_eq!(tokens[2].token_type(), &TokenType::STAR);
    assert_eq!(tokens[3].token_type(), &TokenType::SLASH);
}

#[test]
fn test_nepali_operators() {
    let mut lexer = Lexer::new("joda ghatau ulto".as_bytes());
    let tokens = lexer.tokenize();

    // Three operators + EOF token
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type(), &TokenType::PLUS);
    assert_eq!(tokens[1].token_type(), &TokenType::MINUS);
    assert_eq!(tokens[2].token_type(), &TokenType::BANG);
}

#[test]
fn test_string_literal() {
    let mut lexer = Lexer::new("\"hello world\"".as_bytes());
    let tokens = lexer.tokenize();

    // One string token + EOF token
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type(), &TokenType::STRING);
    assert_eq!(tokens[0].lexeme(), "hello world");
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new("\"hello".as_bytes());
    // We don't care about tokens here, just errors
    lexer.tokenize();

    assert_eq!(lexer.get_errors().len(), 1);
    assert!(matches!(
        lexer.get_errors()[0],
        LexError::UnterminatedString(_)
    ));
}

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("".as_bytes());
    let tokens = lexer.tokenize();

    // Only EOF token
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type(), &TokenType::EOF);
}

#[test]
fn test_complex_expression() {
    let mut lexer = Lexer::new("let x = 10 + 20;".as_bytes());
    let tokens = lexer.tokenize();

    // Test the sequence of tokens
    assert_eq!(tokens[0].token_type(), &TokenType::VAR); // let
    assert_eq!(tokens[1].token_type(), &TokenType::IDENTIFIER); // x
    assert_eq!(tokens[2].token_type(), &TokenType::EQUAL); // =
    assert_eq!(tokens[3].token_type(), &TokenType::NUMBER); // 10
    assert_eq!(tokens[4].token_type(), &TokenType::PLUS); // +
    assert_eq!(tokens[5].token_type(), &TokenType::NUMBER); // 20
    assert_eq!(tokens[6].token_type(), &TokenType::SEMICOLON); // ;
}

#[test]
fn test_complex_nepali_expression() {
    let mut lexer = Lexer::new("manum x bhaneko 10 joda 20;".as_bytes());
    let tokens = lexer.tokenize();

    // Test the sequence of tokens
    assert_eq!(tokens[0].token_type(), &TokenType::VAR); // let
    assert_eq!(tokens[1].token_type(), &TokenType::IDENTIFIER); // x
    assert_eq!(tokens[2].token_type(), &TokenType::EQUAL); // =
    assert_eq!(tokens[3].token_type(), &TokenType::NUMBER); // 10
    assert_eq!(tokens[4].token_type(), &TokenType::PLUS); // +
    assert_eq!(tokens[5].token_type(), &TokenType::NUMBER); // 20
    assert_eq!(tokens[6].token_type(), &TokenType::SEMICOLON); // ;
}

#[test]
fn test_comment_handling() {
    let mut lexer = Lexer::new("let x = 10; // this is a comment".as_bytes());
    let tokens = lexer.tokenize();

    // Comment should be ignored, expect 6 tokens
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type(), &TokenType::VAR); // let
    assert_eq!(tokens[1].token_type(), &TokenType::IDENTIFIER); // x
}

#[test]
fn test_multiline_comments() {
    let input = r#"/* this
	is
		multi
			line
				comment*/"#;

    let mut lexer = Lexer::new(input.as_bytes());
    let tokens = lexer.tokenize();

    // Only the EOF token should be present
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type(), &TokenType::EOF);
}

#[test]
fn test_mix_language_code() {
    let input = r#"/* this
	is
		multi
			line
				comment
    */
    manum x bhaneko 10;     // Declares a variable in Nepali
    let y = 5;              // Declares a variable in English
    if (x > y) {            // Uses English for conditionals
        // this is a comment in the middle of code
        dekhau("Satya");    // Prints a message in Nepali
    } athwa {               // Uses Nepali for else condition
        /* this is multi
        line comment in the middle of code
         , is this working ahha
        */
        print("false");     // Another print in English
    }"#;

    let tokens = Lexer::new(input.as_bytes()).tokenize();

    assert_eq!(tokens.len(), 32);

    assert_eq!(tokens[0].token_type(), &TokenType::VAR); // manum
    assert_eq!(tokens[1].token_type(), &TokenType::IDENTIFIER); // x
    assert_eq!(tokens[2].token_type(), &TokenType::EQUAL); // bhaneko
    assert_eq!(tokens[3].token_type(), &TokenType::NUMBER); // 10
    assert_eq!(tokens[4].token_type(), &TokenType::SEMICOLON); // ;

    assert_eq!(tokens[5].token_type(), &TokenType::VAR); // let
    assert_eq!(tokens[6].token_type(), &TokenType::IDENTIFIER); // y
    assert_eq!(tokens[7].token_type(), &TokenType::EQUAL); // =
    assert_eq!(tokens[8].token_type(), &TokenType::NUMBER); // 5
    assert_eq!(tokens[9].token_type(), &TokenType::SEMICOLON); // ;

    assert_eq!(tokens[10].token_type(), &TokenType::IF); // if
    assert_eq!(tokens[11].token_type(), &TokenType::LEFTPAREN); // (
    assert_eq!(tokens[12].token_type(), &TokenType::IDENTIFIER); // x
    assert_eq!(tokens[13].token_type(), &TokenType::GREATER); // >
    assert_eq!(tokens[14].token_type(), &TokenType::IDENTIFIER); // y
    assert_eq!(tokens[15].token_type(), &TokenType::RIGHTPAREN); // )
    assert_eq!(tokens[16].token_type(), &TokenType::LEFTBRACE); // {

    assert_eq!(tokens[17].token_type(), &TokenType::PRINT); // dekhau
    assert_eq!(tokens[18].token_type(), &TokenType::LEFTPAREN); // (
    assert_eq!(tokens[19].token_type(), &TokenType::STRING); // "Satya"
    assert_eq!(tokens[20].token_type(), &TokenType::RIGHTPAREN); // )
    assert_eq!(tokens[21].token_type(), &TokenType::SEMICOLON); // ;

    assert_eq!(tokens[22].token_type(), &TokenType::RIGHTBRACE); // }
    assert_eq!(tokens[23].token_type(), &TokenType::ELSE); // athwa
    assert_eq!(tokens[24].token_type(), &TokenType::LEFTBRACE); // {

    assert_eq!(tokens[25].token_type(), &TokenType::PRINT); // print
    assert_eq!(tokens[26].token_type(), &TokenType::LEFTPAREN); // (
    assert_eq!(tokens[27].token_type(), &TokenType::STRING); // "false"
    assert_eq!(tokens[28].token_type(), &TokenType::RIGHTPAREN); // )
    assert_eq!(tokens[29].token_type(), &TokenType::SEMICOLON); // ;

    assert_eq!(tokens[30].token_type(), &TokenType::RIGHTBRACE); // }

    assert_eq!(tokens.last().unwrap().token_type(), &TokenType::EOF);
}

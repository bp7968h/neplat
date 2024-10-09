pub enum TokenType {
    // Literals
    STRING,
    NUMBER,
    IDENTIFIER,

    //Keywords
    TRUE,
    FALSE,
    AND,
    OR,
    IF,
    ELSE,
    FUNC,
    RETURN,
    FOR,
    NULL,
    PRINT,
    VAR,
    WHILE,
    CLASS,
    THIS,
    SUPER,

    //Characters - Single
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMI_COLON,
    SLASH,
    STAR,

    //Characters - Double
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    EOF
}
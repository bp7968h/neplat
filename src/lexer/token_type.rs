#[derive(Debug, Clone, PartialEq)]
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
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    //Characters - Double
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    EOF
}
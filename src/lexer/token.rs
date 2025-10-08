use std::fmt;

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum TokenType {
    Hash,
    Star,
    DoubleStar,
    Underscore,
    Dash,
    Tilde,
    Content,
    NewLine,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}) {}", self.token_type, self.lexeme)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

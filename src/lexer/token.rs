use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    Hash,
    Star,
    Underscore,
    Dash,
    Tilde,
    Content,
    NewLine,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
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

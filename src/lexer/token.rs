use std::fmt;

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum TokenType {
    // layout
    Hash,
    Dash,
    NewLine,
    BreakLine,
    // inline
    Delimiter {
        length: usize,
        can_open: bool,
        can_close: bool,
    },
    OpenEmphasis,
    CloseEmphasis,
    OpenStrong,
    CloseStrong,
    Tilde,
    OpeningBracket,
    ClosingBracket,
    OpeningParenthesis,
    ClosingParenthesis,
    // content
    Content,
    Whitespace,
    // meta
    Eof,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub pos: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?} {})",
            self.token_type,
            self.lexeme.escape_default()
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, pos: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            pos,
        }
    }
}

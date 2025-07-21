use crate::lexer::{ErrorHandler, Token, TokenType};

pub struct Lexer<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Lexer<'a> {
    pub fn new(source: String, error_handler: &'a mut ErrorHandler) -> Self {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            error_handler: error_handler,
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '#' => self.add_token(TokenType::Hash),
            '*' => self.add_token(TokenType::Star),
            '_' => self.add_token(TokenType::Underscore),
            '-' => self.add_token(TokenType::Dash),
            '~' => self.add_token(TokenType::Tilde),
            ' ' => {}
            '\t' => {}
            '\n' => {
                self.add_token(TokenType::NewLine);
                self.line += 1;
            }
            _ => {
                if !self.is_special(c) {
                    self.scan_text();
                } else {
                    self.error_handler
                        .log_error(self.line, self.current, "Unexpected character.");
                }
            }
        }
    }

    fn scan_text(&mut self) {
        while !self.is_at_end() {
            let c = self.peak();

            if self.is_special(c) {
                break;
            }

            self.advance();
        }
        self.add_token(TokenType::Content);
    }

    fn is_special(&self, c: char) -> bool {
        matches!(c, '#' | '*' | '_' | '-' | '~' | '\0')
    }

    fn peak(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.peak();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }
}

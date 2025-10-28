use crate::lexer::{Token, TokenType};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '#' => self.add_token(TokenType::Hash),
            '*' => {
                if self.matching('*') {
                    self.add_token(TokenType::DoubleStar);
                } else {
                    self.add_token(TokenType::Star)
                }
            }
            '_' => self.add_token(TokenType::Underscore),
            '-' => {
                if self.matching(' ') {
                    self.add_token(TokenType::Dash)
                } else {
                    self.scan_text();
                }
            }
            '~' => self.add_token(TokenType::Tilde),
            ' ' => {}
            '\t' => {}
            '\n' => {
                self.add_token(TokenType::NewLine);
                self.line += 1;
            }

            _ => {
                if !self.is_content_closing(c) {
                    self.scan_text();
                }
            }
        }
    }

    fn scan_text(&mut self) {
        while !self.is_at_end() {
            let c = self.peak();

            if self.is_content_closing(c) {
                break;
            }

            self.advance();
        }
        self.add_token(TokenType::Content);
    }

    fn is_content_closing(&mut self, c: char) -> bool {
        match c {
            '\0' | '\n' | '*' | '_' | '-' | '~' => true,
            '#' => self.matching(' '),
            _ => false,
        }
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

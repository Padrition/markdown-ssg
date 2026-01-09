use crate::lexer::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Lexer {
            source: chars,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '#' => self.add_token(TokenType::Hash),
            '_' => self.add_token(TokenType::Underscore),
            '[' => self.add_token(TokenType::OpeningBracket),
            ']' => self.add_token(TokenType::ClosingBracket),
            '(' => self.add_token(TokenType::OpeningParenthesis),
            ')' => self.add_token(TokenType::ClosingParenthesis),
            ' ' | '\t' => {
                self.scan_whitespace();
            }
            '\n' => {
                self.add_token(TokenType::NewLine);
                self.line += 1;
            }
            '*' => {
                if self.matching('*') {
                    self.add_token(TokenType::DoubleStar);
                } else {
                    self.add_token(TokenType::Star)
                }
            }
            '-' => {
                if self.matching(' ') {
                    self.add_token(TokenType::Dash)
                } else {
                    self.scan_text();
                }
            }
            '~' => {
                if self.matching('~') {
                    self.add_token(TokenType::Tilde)
                } else {
                    self.scan_text();
                }
            }

            _ => {
                if !self.is_content_closing(c) {
                    self.scan_text();
                }
            }
        }
    }

    fn scan_whitespace(&mut self) {
        while matches!(self.peak(), ' ' | '\t') && !self.is_at_end() {
            self.advance();
        }

        self.add_token(TokenType::Whitespace);
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
            '\0' | '\n' | '*' | '_' | '-' | '~' | '[' | ']' | '(' | ')' => true,
            '#' => self.matching(' '),
            _ => false,
        }
    }

    fn peak(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_owned(),
            self.line,
            self.start,
        ));
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
        let text: String = self.source[self.start..self.current].iter().collect();

        let final_text = if token_type == TokenType::Whitespace {
            " ".to_owned()
        } else {
            text
        };

        self.tokens
            .push(Token::new(token_type, final_text, self.line, self.start));
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }
}

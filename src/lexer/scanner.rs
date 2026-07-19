use crate::lexer::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_punctuation(c: char) -> bool {
    c.is_ascii_punctuation()
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Scanner {
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
            '[' => self.add_token(TokenType::OpeningBracket),
            ']' => self.add_token(TokenType::ClosingBracket),
            '(' => self.add_token(TokenType::OpeningParenthesis),
            ')' => self.add_token(TokenType::ClosingParenthesis),
            '`' => self.add_token(TokenType::Backtick),
            ' ' | '\t' => self.add_token(TokenType::Whitespace),
            '\n' => {
                if self.current > 2
                    && self.source[self.current - 3] == ' '
                    && self.source[self.current - 2] == ' '
                {
                    self.add_token(TokenType::BreakLine);
                } else {
                    self.add_token(TokenType::NewLine);
                }
                self.line += 1;
            }
            '-' => {
                if self.is_horizontal_rule('-') {
                    self.add_token(TokenType::HorizontalRule);
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else if self.matching(' ') {
                    self.add_token(TokenType::Dash);
                } else {
                    self.scan_text();
                }
            }
            '~' => {
                if self.matching('~') {
                    self.add_token(TokenType::Tilde);
                } else {
                    self.scan_text();
                }
            }
            '*' | '_' => {
                if self.current == 1 && self.is_horizontal_rule(c) {
                    self.add_token(TokenType::HorizontalRule);

                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }

                    return;
                }
                let marker = c;
                let mut length = 1;

                while self.peek() == marker {
                    self.advance();
                    length += 1;
                }

                let prev = if self.start == 0 {
                    None
                } else {
                    Some(self.source[self.start - 1])
                };

                let next = if self.is_at_end() {
                    None
                } else {
                    Some(self.peek())
                };

                let prev_is_whitespace = prev.map_or(true, is_whitespace);
                let prev_is_punctuation = prev.map_or(false, is_punctuation);

                let next_is_whitespace = next.map_or(true, is_whitespace);
                let next_is_punctuation = next.map_or(false, is_punctuation);

                let left_flanking = !next_is_whitespace
                    && (!next_is_punctuation || prev_is_whitespace || prev_is_punctuation);

                let right_flanking = !prev_is_whitespace
                    && (!prev_is_punctuation || next_is_whitespace || next_is_punctuation);

                let can_open = if marker == '*' {
                    left_flanking
                } else {
                    left_flanking && !(right_flanking && !prev_is_punctuation)
                };

                let can_close = if marker == '*' {
                    right_flanking
                } else {
                    right_flanking && !(left_flanking && !next_is_punctuation)
                };

                self.add_token(TokenType::Delimiter {
                    length,
                    can_open,
                    can_close,
                });
            }
            _ => {
                if !self.is_content_closing(c) {
                    self.scan_text();
                }
            }
        }
    }

    fn is_horizontal_rule(&self, marker: char) -> bool {
        let mut count = 1;
        let mut i = self.current;
        while i < self.source.len() {
            let ch = self.source[i];
            if ch == '\n' {
                break;
            }
            if ch == marker {
                count += 1;
            } else if ch != ' ' {
                return false;
            }
            i += 1;
        }

        count >= 3
    }

    fn scan_text(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();

            if self.is_content_closing(c) {
                break;
            }

            self.advance();
        }
        self.add_token(TokenType::Content);
    }

    fn is_content_closing(&mut self, c: char) -> bool {
        match c {
            '\0' | '\n' | '*' | '_' | '-' | '~' | '[' | ']' | '(' | ')' | '`' => true,
            '#' => self.matching(' '),
            _ => false,
        }
    }

    fn peek(&self) -> char {
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
            TokenType::Eof,
            String::new(),
            self.line,
            self.start,
        ));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
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

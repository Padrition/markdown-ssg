use crate::lexer::{Token, TokenType};
use crate::parser::{in_line_node::InLineNode, markdown_node::MarkdownNode};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<MarkdownNode> {
        self.document()
    }

    fn document(&mut self) -> Vec<MarkdownNode> {
        self.block()
    }

    fn block(&mut self) -> Vec<MarkdownNode> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            if self.match_tokens(&[TokenType::Hash]) {
                nodes.push(self.heading());
            } else if self.match_tokens(&[TokenType::NewLine]) {
                self.line_breaks();
            } else {
                nodes.push(self.paragraph());
            }
        }

        nodes
    }

    fn line_breaks(&mut self) {
        while !self.is_at_end() && self.match_tokens(&[TokenType::NewLine]) {
            // consume new lines
        }
    }

    fn heading(&mut self) -> MarkdownNode {
        let mut level = 1;
        while self.match_tokens(&[TokenType::Hash]) {
            level += 1;
        }

        let content = self.in_line_until(&[TokenType::NewLine]);
        //skip new line token
        self.advance();

        MarkdownNode::Heading {
            level: level,
            content: content,
        }
    }

    fn paragraph(&mut self) -> MarkdownNode {
        let mut nodes = Vec::new();
        while !self.is_at_end() && !self.match_tokens(&[TokenType::NewLine]) {
            let mut content = self.in_line_until(&[TokenType::NewLine]);
            //skip new line token
            self.advance();

            if !self.is_at_end() && !self.peek_match_tokens(&[TokenType::NewLine]) {
                self.push_text_node(&mut content, " ".to_string());
            }

            nodes.append(&mut content);
        }

        MarkdownNode::Paragraph(nodes)
    }

    fn in_line_until(&mut self, stop_tokens: &[TokenType]) -> Vec<InLineNode> {
        let mut nodes = Vec::new();

        while !self.is_at_end() && !self.peek_match_tokens(stop_tokens) {
            if self.match_tokens(&[TokenType::DoubleStar]) {
                let content = self.in_line_until(&[TokenType::DoubleStar]);
                self.consume(TokenType::DoubleStar, "Expected a closing **");
                nodes.push(InLineNode::Strong(content));
            } else if self.match_tokens(&[TokenType::Star]) {
                let content = self.in_line_until(&[TokenType::Star]);
                self.consume(TokenType::Star, "Expected a closing *");
                nodes.push(InLineNode::Emphasis(content));
            } else if self.match_tokens(&[TokenType::Tilde]) {
                let content = self.in_line_until(&[TokenType::Tilde]);
                self.consume(TokenType::Tilde, "Expected a closing ~");
                nodes.push(InLineNode::Strikethrough(content));
            } else {
                let token = if self.peek_match_tokens(&[TokenType::Whitespace]) {
                    self.consume(TokenType::Whitespace, "Expected whitespace")
                } else {
                    self.consume(TokenType::Content, "Expected content")
                };

                self.push_text_node(&mut nodes, token.lexeme);
            }
        }

        nodes
    }

    fn push_text_node(&mut self, nodes: &mut Vec<InLineNode>, lexeme: String) {
        if lexeme.trim().is_empty() && nodes.is_empty() {
            return;
        }

        if let Some(InLineNode::Text(last_text)) = nodes.last_mut() {
            last_text.push_str(&lexeme);
        } else {
            nodes.push(InLineNode::Text(lexeme));
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        let token = self.peek();

        panic!("{} at char: {} line: {}", msg, token.line, token.pos);
    }

    fn peek_match_tokens(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(*t)) {
            return true;
        }
        return false;
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(*t)) {
            self.advance();
            return true;
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if !self.is_at_end() {
            return self.peek().token_type == token_type;
        }

        return false;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Token {
        return self.tokens.iter().nth(self.current).unwrap().clone();
    }

    fn previous(&self) -> Token {
        return self.tokens.iter().nth(&self.current - 1).unwrap().clone();
    }
}

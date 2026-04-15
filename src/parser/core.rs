use log::debug;

use crate::lexer::{Token, TokenType};
use crate::parser::{in_line_node::InLineNode, markdown_node::MarkdownNode};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

struct StackDelimiter {
    position: usize,
    length: usize,
    can_open: bool,
}

#[derive(PartialEq)]
enum EmphasisKind {
    Emphasis,
    Strong,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<MarkdownNode> {
        self.document()
    }

    fn document(&mut self) -> Vec<MarkdownNode> {
        self.parse_delimiters();
        self.block()
    }

    fn parse_delimiters(&mut self) {
        let mut stack: Vec<StackDelimiter> = Vec::new();

        let mut i = 0;
        while i < self.tokens.len() {
            if let TokenType::Delimiter {
                mut length,
                can_open,
                can_close,
            } = self.tokens[i].token_type
            {
                if can_open {
                    stack.push(StackDelimiter {
                        position: i,
                        length,
                        can_open,
                    });
                }

                if can_close {
                    while length != 0 {
                        let mut j = stack.len();
                        if j > 0 {
                            j -= 1;
                            // If the delimiter can both open and close, we take the one that can only open
                            let opener_stack_position = if can_open && j != 0 { j - 1 } else { j };
                            let opener_length = stack[opener_stack_position].length;
                            let opener_position = stack[opener_stack_position].position;
                            let opener_can_open = stack[opener_stack_position].can_open;

                            if opener_position == i {
                                break;
                            }

                            if opener_can_open {
                                let use_len = if length >= 2 && opener_length >= 2 {
                                    2
                                } else {
                                    1
                                };

                                match use_len {
                                    2 => self.apply_emphasis(
                                        opener_position,
                                        EmphasisKind::Strong,
                                        &mut i,
                                    ),
                                    1 => self.apply_emphasis(
                                        opener_position,
                                        EmphasisKind::Emphasis,
                                        &mut i,
                                    ),
                                    _ => unreachable!(),
                                }

                                if can_open {
                                    stack[j].length -= use_len;
                                    if stack[j].length == 0 {
                                        stack.remove(j);
                                    }
                                }

                                stack[opener_stack_position].length -= use_len;
                                if stack[opener_stack_position].length == 0 {
                                    stack.remove(opener_stack_position);
                                }

                                length -= use_len;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        debug!(
            "Pre-parser output:\n{}\n",
            self.tokens
                .iter()
                .map(|t| t.to_string())
                .collect::<String>()
        );
    }

    fn apply_emphasis(&mut self, opener_pos: usize, kind: EmphasisKind, i: &mut usize) {
        let (open, close) = match kind {
            EmphasisKind::Emphasis => (TokenType::OpenEmphasis, TokenType::CloseEmphasis),
            EmphasisKind::Strong => (TokenType::OpenStrong, TokenType::CloseStrong),
        };

        //open
        let mut new_token = self.tokens[opener_pos].clone();
        new_token.token_type = open;
        new_token.lexeme = if kind == EmphasisKind::Strong {
            String::from("**")
        } else {
            String::from("*")
        };

        if !matches!(
            self.tokens[opener_pos].token_type,
            TokenType::OpenStrong
                | TokenType::OpenEmphasis
                | TokenType::CloseEmphasis
                | TokenType::CloseStrong
        ) {
            self.tokens[opener_pos] = new_token;
        } else {
            let new_pos = if matches!(
                self.tokens[opener_pos].token_type,
                TokenType::CloseEmphasis | TokenType::CloseStrong
            ) {
                opener_pos + 1
            } else {
                opener_pos
            };
            self.tokens.insert(new_pos, new_token);
            *i += 1;
        }

        //close
        if matches!(self.tokens[*i].token_type, TokenType::Delimiter { .. }) {
            self.tokens[*i].token_type = close;
        } else {
            new_token = self.tokens[*i].clone();
            new_token.token_type = close;
            new_token.lexeme = if kind == EmphasisKind::Strong {
                String::from("**")
            } else {
                String::from("*")
            };

            self.tokens.insert(*i, new_token);
            *i += 1;
        }
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

        MarkdownNode::Heading { level, content }
    }

    fn paragraph(&mut self) -> MarkdownNode {
        let mut nodes = Vec::new();
        while !self.is_at_end() && !self.match_tokens(&[TokenType::NewLine]) {
            let mut content = self.in_line_until(&[TokenType::NewLine]);
            //skip new line token
            self.advance();

            if !self.is_at_end() && !self.peek_match_tokens(&[TokenType::NewLine]) {
                self.push_text_node(&mut content, " ".to_owned());
            }

            nodes.append(&mut content);
        }

        MarkdownNode::Paragraph(nodes)
    }

    fn in_line_until(&mut self, stop_tokens: &[TokenType]) -> Vec<InLineNode> {
        let mut nodes = Vec::new();

        while !self.is_at_end() && !self.peek_match_tokens(stop_tokens) {
            let node = if self.match_tokens(&[TokenType::OpenStrong]) {
                self.parse_strong()
            } else if self.match_tokens(&[TokenType::OpenEmphasis]) {
                self.parse_emphasis()
            } else if self.match_tokens(&[TokenType::Tilde]) {
                self.parse_strikethrough()
            } else if self.match_tokens(&[TokenType::OpeningBracket]) {
                self.parse_link()
            } else if self.match_tokens(&[TokenType::BreakLine]) {
                InLineNode::BreakLine
            } else {
                self.parse_text()
            };
            if let InLineNode::Text(lexeme) = node {
                self.push_text_node(&mut nodes, lexeme);
            } else {
                nodes.push(node);
            }
        }

        nodes
    }

    fn parse_strong(&mut self) -> InLineNode {
        let content = self.in_line_until(&[TokenType::CloseStrong]);
        self.consume(TokenType::CloseStrong, "Expected a closing **");
        InLineNode::Strong(content)
    }

    fn parse_emphasis(&mut self) -> InLineNode {
        let content = self.in_line_until(&[TokenType::CloseEmphasis]);
        self.consume(TokenType::CloseEmphasis, "Expected a closing *");
        InLineNode::Emphasis(content)
    }

    fn parse_strikethrough(&mut self) -> InLineNode {
        let content = self.in_line_until(&[TokenType::Tilde]);
        self.consume(TokenType::Tilde, "Expected a closing ~");
        InLineNode::Strikethrough(content)
    }

    fn parse_link(&mut self) -> InLineNode {
        let content = self.in_line_until(&[TokenType::ClosingBracket]);
        self.consume(TokenType::ClosingBracket, "Expected a ]");

        self.consume(TokenType::OpeningParenthesis, "Expected a (");

        let dest = self.collect_until(&[TokenType::ClosingParenthesis]);
        self.consume(TokenType::ClosingParenthesis, "Expected a )");

        InLineNode::Link { content, dest }
    }

    fn parse_text(&mut self) -> InLineNode {
        let token = if self.peek_match_tokens(&[TokenType::Whitespace]) {
            self.consume(TokenType::Whitespace, "Expected whitespace")
        } else if let TokenType::Delimiter { .. } = self.peek().token_type {
            self.advance()
        } else {
            self.consume(TokenType::Content, "Expected content")
        };
        InLineNode::Text(token.lexeme)
    }

    fn collect_until(&mut self, stop_tokens: &[TokenType]) -> String {
        let mut result = String::new();
        while !self.is_at_end() && !self.peek_match_tokens(stop_tokens) {
            result.push_str(&self.advance().lexeme);
        }
        result
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

        panic!("{} at char: {} line: {}", msg, token.pos, token.line);
    }

    fn peek_match_tokens(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(*t)) {
            return true;
        }
        false
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(*t)) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if !self.is_at_end() {
            return self.peek().token_type == token_type;
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .unwrap_or(&Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: self.current,
                pos: 0,
            })
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(&self.current - 1).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn make_token(token_type: TokenType, lexeme: &str) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: 1,
            pos: 0,
        }
    }

    fn parse_inline(tokens: Vec<Token>) -> Vec<MarkdownNode> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    fn paragraph(nodes: Vec<InLineNode>) -> MarkdownNode {
        MarkdownNode::Paragraph(nodes)
    }

    fn text(s: &str) -> InLineNode {
        InLineNode::Text(s.to_string())
    }

    #[test]
    fn test_strong() {
        let tokens = vec![
            make_token(TokenType::OpenStrong, "**"),
            make_token(TokenType::Content, "bold"),
            make_token(TokenType::CloseStrong, "**"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strong(vec![text("bold")])])]
        );
    }

    #[test]
    fn test_emphasis_with_no_eof_token() {
        let tokens = vec![
            make_token(TokenType::OpenEmphasis, "*"),
            make_token(TokenType::Content, "em"),
            make_token(TokenType::CloseEmphasis, "*"),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Emphasis(vec![text("em")])])]
        );
    }

    #[test]
    fn test_strikethrough() {
        let tokens = vec![
            make_token(TokenType::Tilde, "~"),
            make_token(TokenType::Content, "content"),
            make_token(TokenType::Tilde, "~"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strikethrough(vec![text(
                "content"
            )])])]
        );
    }

    #[test]
    fn test_strikethrough_with() {
        let tokens = vec![
            make_token(TokenType::Tilde, "~"),
            make_token(TokenType::Content, "content"),
            make_token(TokenType::Tilde, "~"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strikethrough(vec![text(
                "content"
            )])])]
        );
    }

    #[test]
    fn test_link() {
        let tokens = vec![
            make_token(TokenType::OpeningBracket, "["),
            make_token(TokenType::Content, "link content"),
            make_token(TokenType::ClosingBracket, "]"),
            make_token(TokenType::OpeningParenthesis, "("),
            make_token(TokenType::ClosingBracket, "link"),
            make_token(TokenType::ClosingParenthesis, ")"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Link {
                content: vec![text("link content")],
                dest: "link".to_string()
            }])]
        );
    }

    #[test]
    fn test_br() {
        let tokens = vec![
            make_token(TokenType::BreakLine, "\n"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(nodes, vec![paragraph(vec![InLineNode::BreakLine])]);
    }

    #[test]
    fn test_text() {
        let tokens = vec![
            make_token(TokenType::Whitespace, ""),
            make_token(TokenType::Whitespace, ""),
            make_token(TokenType::Content, "content"),
            make_token(TokenType::Eof, ""),
        ];
        let nodes = parse_inline(tokens);
        assert_eq!(nodes, vec![paragraph(vec![text("content")])]);
    }
}

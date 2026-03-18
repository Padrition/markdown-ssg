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
            if self.match_tokens(&[TokenType::OpenStrong]) {
                let content = self.in_line_until(&[TokenType::CloseStrong]);
                self.consume(TokenType::CloseStrong, "Expected a closing **");
                nodes.push(InLineNode::Strong(content));
            } else if self.match_tokens(&[TokenType::OpenEmphasis]) {
                let content = self.in_line_until(&[TokenType::CloseEmphasis]);
                self.consume(TokenType::CloseEmphasis, "Expected a closing *");
                nodes.push(InLineNode::Emphasis(content));
            } else if self.match_tokens(&[TokenType::Tilde]) {
                let content = self.in_line_until(&[TokenType::Tilde]);
                self.consume(TokenType::Tilde, "Expected a closing ~");
                nodes.push(InLineNode::Strikethrough(content));
            } else if self.match_tokens(&[TokenType::OpeningBracket]) {
                let content = self.in_line_until(&[TokenType::ClosingBracket]);
                self.consume(TokenType::ClosingBracket, "Expected a ]");

                self.consume(TokenType::OpeningParenthesis, "Expected a (");

                let mut dest = String::new();
                while !self.is_at_end() && !self.peek_match_tokens(&[TokenType::ClosingParenthesis])
                {
                    let token = self.advance();
                    dest.push_str(&token.lexeme);
                }
                self.consume(TokenType::ClosingParenthesis, "Expected a )");

                nodes.push(InLineNode::Link { content, dest });
            } else if self.match_tokens(&[TokenType::BreakLine]) {
                nodes.push(InLineNode::BreakLine);
            } else if let TokenType::Delimiter { .. } = self.peek().token_type {
                let token = self.advance();

                self.push_text_node(&mut nodes, token.lexeme);
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
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(&self.current - 1).unwrap().clone()
    }
}

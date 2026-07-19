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
                length,
                can_open,
                can_close,
            } = self.tokens[i].token_type
            {
                stack.push(StackDelimiter {
                    position: i,
                    length,
                    can_open,
                });

                if can_close {
                    let j = stack.len() - 1;
                    let opener_stack_position = j - 1;
                    let opener_length = stack[opener_stack_position].length;
                    let opener_position = stack[opener_stack_position].position;
                    let opener_can_open = stack[opener_stack_position].can_open;

                    if opener_position == i {
                        i += 1;
                        continue;
                    }

                    if opener_can_open {
                        let use_len = if length >= 2 && opener_length >= 2 {
                            2
                        } else {
                            1
                        };

                        let kind = match use_len {
                            2 => EmphasisKind::Strong,
                            1 => EmphasisKind::Emphasis,
                            _ => unreachable!(),
                        };

                        let shift = self.apply_emphasis(opener_position, kind, &mut i);

                        for delimiter in &mut stack {
                            if delimiter.position > opener_position {
                                delimiter.position += 1 + shift;
                            }
                        }

                        stack[j].length -= use_len;

                        self.tokens[stack[j].position].lexeme = self.tokens[stack[j].position]
                            .lexeme
                            .chars()
                            .take(stack[j].length)
                            .collect();

                        if stack[j].length == 0 {
                            self.tokens.remove(stack[j].position);
                            for delimiter in &mut stack[j + 1..] {
                                delimiter.position -= 1;
                            }
                            i -= 1;
                            stack.remove(j);
                        }

                        stack[opener_stack_position].length -= use_len;
                        if stack[opener_stack_position].length == 0 {
                            self.tokens.remove(stack[opener_stack_position].position);
                            for delimiter in &mut stack[opener_stack_position + 1..] {
                                delimiter.position -= 1;
                            }
                            i -= 1;
                            stack.remove(opener_stack_position);
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
                .map(ToString::to_string)
                .collect::<String>()
        );
    }

    fn apply_emphasis(&mut self, opener_pos: usize, kind: EmphasisKind, i: &mut usize) -> usize {
        let (open, close) = match kind {
            EmphasisKind::Emphasis => (TokenType::OpenEmphasis, TokenType::CloseEmphasis),
            EmphasisKind::Strong => (TokenType::OpenStrong, TokenType::CloseStrong),
        };

        //open
        let mut shift = 0;

        let mut new_token = self.tokens[opener_pos].clone();
        new_token.token_type = open;
        new_token.lexeme = if kind == EmphasisKind::Strong {
            String::from("**")
        } else {
            String::from("*")
        };

        let new_opener_pos = if matches!(
            self.tokens[opener_pos + 1].token_type,
            TokenType::CloseEmphasis | TokenType::CloseStrong
        ) {
            shift += 1;
            opener_pos + 2
        } else {
            opener_pos + 1
        };

        shift += 1;
        self.tokens.insert(new_opener_pos, new_token);
        *i += 1;

        //close
        new_token = self.tokens[*i].clone();
        new_token.token_type = close;
        new_token.lexeme = if kind == EmphasisKind::Strong {
            String::from("**")
        } else {
            String::from("*")
        };

        self.tokens.insert(*i, new_token);
        *i += 1;

        shift
    }

    fn block(&mut self) -> Vec<MarkdownNode> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            if self.match_tokens(&[TokenType::Hash]) {
                nodes.push(self.heading());
            } else if self.match_tokens(&[TokenType::NewLine]) {
                self.line_breaks();
            } else if self.match_tokens(&[TokenType::HorizontalRule]) {
                nodes.push(MarkdownNode::HorizontalRule);
            } else if self.count_consecutive_tokens(self.current, TokenType::Backtick) >= 3 {
                nodes.push(self.parse_code_block())
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
            let mut content = self.in_line_until(&[TokenType::NewLine, TokenType::HorizontalRule]);

            let is_line_break = self.peek().token_type == TokenType::NewLine
                && self.peek_next().token_type == TokenType::NewLine;

            if is_line_break {
                self.advance();
            }

            if !self.is_at_end() && !is_line_break {
                push_text_node(&mut content, " ".to_owned());
            }

            nodes.append(&mut content);
        }

        MarkdownNode::Paragraph(nodes)
    }

    fn parse_code_block(&mut self) -> MarkdownNode {
        let opening_count = self.count_consecutive_tokens(self.current, TokenType::Backtick);

        self.current += opening_count;

        let mut language = String::new();
        while !self.is_at_end() && self.tokens[self.current].token_type != TokenType::NewLine {
            language.push_str(&self.tokens[self.current].lexeme);
            self.current += 1;
        }
        if !self.is_at_end() {
            self.current += 1;
        }

        let mut found_match = false;
        let mut match_idx = self.current;

        let start_token_idx = self.current;
        let mut closing_token_ids = 0;

        while match_idx < self.tokens.len() {
            if self.tokens[match_idx].token_type == TokenType::Backtick
                && self.tokens[match_idx - 1].token_type == TokenType::NewLine
            {
                let mut second_count = 0;
                let mut inner_match_idx = match_idx;

                while inner_match_idx < self.tokens.len()
                    && self.tokens[inner_match_idx].token_type == TokenType::Backtick
                {
                    second_count += 1;
                    inner_match_idx += 1;
                }

                if second_count == opening_count {
                    found_match = true;
                    self.current = inner_match_idx;
                    closing_token_ids = match_idx - 1;
                    break;
                } else {
                    match_idx = inner_match_idx;
                }
            } else {
                match_idx += 1;
            }
        }

        if found_match {
            let content = self.tokens[start_token_idx..closing_token_ids]
                .iter()
                .map(|t| t.lexeme.as_str())
                .collect::<String>();
            MarkdownNode::Code {
                language: language,
                content: vec![InLineNode::Text(content)],
            }
        } else {
            let fallback_text = "`".repeat(opening_count);
            MarkdownNode::Paragraph(vec![InLineNode::Text(fallback_text)])
        }
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
            } else if self.match_tokens(&[TokenType::Backtick]) {
                self.parse_backtick()
            } else if self.match_tokens(&[TokenType::BreakLine]) {
                InLineNode::BreakLine
            } else {
                self.parse_text()
            };
            if let InLineNode::Text(lexeme) = node {
                push_text_node(&mut nodes, lexeme);
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

    fn parse_backtick(&mut self) -> InLineNode {
        let count = 1 + self.count_consecutive_tokens(self.current, TokenType::Backtick);

        let mut found_match = false;
        let mut match_idx = self.current;

        let start_token_idx = self.current;
        let mut closing_token_ids = 0;

        while match_idx < self.tokens.len() {
            if self.tokens[match_idx].token_type == TokenType::Backtick {
                let mut second_count = 0;
                let mut inner_match_idx = match_idx;

                while inner_match_idx < self.tokens.len()
                    && self.tokens[inner_match_idx].token_type == TokenType::Backtick
                {
                    second_count += 1;
                    inner_match_idx += 1;
                }

                if second_count == count {
                    found_match = true;
                    self.current = inner_match_idx;
                    closing_token_ids = match_idx;
                    break;
                } else {
                    match_idx = inner_match_idx;
                }
            } else {
                match_idx += 1;
            }
        }

        if found_match {
            let content = self.tokens[start_token_idx..closing_token_ids]
                .iter()
                .map(|t| t.lexeme.as_str())
                .collect::<String>();

            let trimmed_content =
                if content.starts_with(' ') && content.ends_with(' ') && content.trim() != "" {
                    content[1..content.len() - 1].to_string()
                } else {
                    content
                };
            InLineNode::Code(vec![InLineNode::Text(trimmed_content)])
        } else {
            let fallback_text = "`".repeat(count);
            InLineNode::Text(fallback_text)
        }
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

    fn count_consecutive_tokens(&self, mut start_idx: usize, token_type: TokenType) -> usize {
        let mut count = 0;
        while start_idx < self.tokens.len() && self.tokens[start_idx].token_type == token_type {
            count += 1;
            start_idx += 1;
        }

        count
    }

    fn collect_until(&mut self, stop_tokens: &[TokenType]) -> String {
        let mut result = String::new();
        while !self.is_at_end() && !self.peek_match_tokens(stop_tokens) {
            result.push_str(&self.advance().lexeme);
        }
        result
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
                lexeme: String::new(),
                line: self.current,
                pos: 0,
            })
            .clone()
    }

    fn peek_next(&self) -> Token {
        self.tokens
            .get(self.current + 1)
            .unwrap_or(&Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                line: self.current,
                pos: 0,
            })
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(&self.current - 1).unwrap().clone()
    }
}

fn push_text_node(nodes: &mut Vec<InLineNode>, lexeme: String) {
    if lexeme.trim().is_empty() && nodes.is_empty() {
        return;
    }

    if let Some(InLineNode::Text(last_text)) = nodes.last_mut() {
        last_text.push_str(&lexeme);
    } else {
        nodes.push(InLineNode::Text(lexeme));
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::scanner::Scanner;
    use std::vec;

    use super::*;

    fn parse_from_lexemes(lexemes: &str) -> Vec<MarkdownNode> {
        let mut scanner = Scanner::new(lexemes);
        let tokens = scanner.scan_tokens();
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
        let nodes = parse_from_lexemes("**bold**");
        let underscore_nodes = parse_from_lexemes("__bold__");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strong(vec![text("bold")])])]
        );
        assert_eq!(
            underscore_nodes,
            vec![paragraph(vec![InLineNode::Strong(vec![text("bold")])])]
        );
    }

    #[test]
    fn test_emphasis() {
        let nodes = parse_from_lexemes("*em*");
        let underscore_nodes = parse_from_lexemes("_em_");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Emphasis(vec![text("em")])])]
        );
        assert_eq!(
            underscore_nodes,
            vec![paragraph(vec![InLineNode::Emphasis(vec![text("em")])])]
        );
    }

    #[test]
    fn test_strikethrough() {
        let nodes = parse_from_lexemes("~~content~~");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strikethrough(vec![text(
                "content"
            )])])]
        );
    }

    #[test]
    fn test_link() {
        let nodes = parse_from_lexemes("[link content](link)");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Link {
                content: vec![text("link content")],
                dest: "link".to_string()
            }])]
        );
    }

    #[test]
    fn test_text() {
        let nodes = parse_from_lexemes("  content");
        assert_eq!(nodes, vec![paragraph(vec![text("content")])]);
    }

    #[test]
    fn test_nested_tokens_strong_in_em() {
        let nodes = parse_from_lexemes("***bi**i*");
        let underscore_nodes = parse_from_lexemes("___bi__i_");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Emphasis(vec![
                InLineNode::Strong(vec![text("bi")]),
                text("i")
            ])])]
        );
        assert_eq!(underscore_nodes, vec![paragraph(vec![text("___bi__i_")])]);
    }

    #[test]
    fn test_nested_tokens_em_in_strong() {
        let nodes = parse_from_lexemes("***bi*b**");
        let underscore_nodes = parse_from_lexemes("___bi_b__");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Strong(vec![
                InLineNode::Emphasis(vec![text("bi")]),
                text("b")
            ])])]
        );
        assert_eq!(underscore_nodes, vec![paragraph(vec![text("___bi_b__")])]);
    }

    #[test]
    fn test_nested_tokens_em_along_strong() {
        let nodes = parse_from_lexemes("*i***bi**");
        let underscore_nodes = parse_from_lexemes("_i___bi__");
        assert_eq!(
            nodes,
            vec![paragraph(vec![
                InLineNode::Emphasis(vec![text("i")]),
                InLineNode::Strong(vec![text("bi")])
            ])]
        );
        assert_eq!(underscore_nodes, vec![paragraph(vec![text("_i___bi__")])]);
    }

    #[test]
    fn test_nested_tokens_strong_along_em() {
        let nodes = parse_from_lexemes("**b***i*");
        let underscore_nodes = parse_from_lexemes("__b___i_");
        assert_eq!(
            nodes,
            vec![paragraph(vec![
                InLineNode::Strong(vec![text("b")]),
                InLineNode::Emphasis(vec![text("i")])
            ])]
        );
        assert_eq!(underscore_nodes, vec![paragraph(vec![text("__b___i_")])]);
    }

    #[test]
    fn test_uneven_number_of_delimiters() {
        let nodes = parse_from_lexemes("*i***bi***");
        let underscore_nodes = parse_from_lexemes("_i___bi___");
        assert_eq!(
            nodes,
            vec![paragraph(vec![
                InLineNode::Emphasis(vec![text("i")]),
                InLineNode::Strong(vec![text("bi")]),
                text("*")
            ])]
        );
        assert_eq!(underscore_nodes, vec![paragraph(vec![text("_i___bi___")])]);
    }

    #[test]
    fn test_horizontal_rule() {
        let nodes = parse_from_lexemes("***");
        let nodes_dashes = parse_from_lexemes("-----");
        let nodes_underscores = parse_from_lexemes("__________ _____");
        assert!(nodes == nodes_dashes && nodes == nodes_underscores);
        assert_eq!(nodes, vec![MarkdownNode::HorizontalRule])
    }

    #[test]
    fn test_doc() {
        let doc = "~~Strikethrough~~

---";
        let nodes = parse_from_lexemes(doc);
        assert_eq!(
            nodes,
            vec![
                paragraph(vec![InLineNode::Strikethrough(vec![text("Strikethrough")])]),
                MarkdownNode::HorizontalRule
            ]
        );
    }

    #[test]
    fn test_backtick() {
        let nodes = parse_from_lexemes("`abc`");
        assert_eq!(
            nodes,
            vec![paragraph(vec![InLineNode::Code(vec!(text("abc")))])]
        );
    }

    #[test]
    fn test_code_block() {
        let lexemes = "```
asdf
    ff
```";
        let expected_output = "asdf
    ff";
        let nodes = parse_from_lexemes(lexemes);
        assert_eq!(
            nodes,
            vec![MarkdownNode::Code {
                language: "".to_string(),
                content: vec![text(expected_output)]
            }]
        );
    }
}

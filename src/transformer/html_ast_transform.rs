use crate::{
    parser::{
        in_line_node::InLineNode, markdown_node::MarkdownNode, node::Node, visitor::NodeVisitor,
    },
    transformer::html_node::{Display, HtmlNode},
};

pub struct HtmlAstTransformer;

impl HtmlAstTransformer {
    pub fn transform(&mut self, node: &MarkdownNode) -> HtmlNode {
        node.accept(self)
    }

    fn transform_in_line(&mut self, nodes: &[InLineNode]) -> Vec<HtmlNode> {
        nodes.iter().map(|n| n.accept(self)).collect()
    }
}

impl NodeVisitor<HtmlNode> for HtmlAstTransformer {
    fn visit_text(&mut self, text: &str) -> HtmlNode {
        HtmlNode::Text(text.to_owned())
    }

    fn visit_emphasis(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("em"),
            attrs: None,
            children: self.transform_in_line(content),
            display: Display::Inline,
        }
    }

    fn visit_strong(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("strong"),
            attrs: None,
            children: self.transform_in_line(content),
            display: Display::Inline,
        }
    }

    fn visit_strikethrough(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("del"),
            attrs: None,
            children: self.transform_in_line(content),
            display: Display::Inline,
        }
    }

    fn visit_heading(&mut self, level: usize, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: format!("h{level}"),
            attrs: None,
            children: self.transform_in_line(content),
            display: Display::Block,
        }
    }

    fn visit_paragraph(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("p"),
            attrs: None,
            children: self.transform_in_line(content),
            display: Display::Block,
        }
    }

    fn visit_link(&mut self, content: &[InLineNode], dest: &str) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("a"),
            attrs: Some(vec![("href".to_owned(), dest.to_owned())]),
            children: self.transform_in_line(content),
            display: Display::Inline,
        }
    }

    fn visit_break_line(&mut self) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("br"),
            attrs: None,
            children: vec![],
            display: Display::Void,
        }
    }

    fn visit_horizontal_rule(&mut self) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("hr"),
            attrs: None,
            children: vec![],
            display: Display::Block,
        }
    }
}

use crate::{
    parser::{
        in_line_node::InLineNode, markdown_node::MarkdownNode, node::Node, visitor::NodeVisitor,
    },
    transformer::html_node::HtmlNode,
};

pub struct HtmlTransformer;

impl HtmlTransformer {
    pub fn transform(&mut self, node: &MarkdownNode) -> HtmlNode {
        node.accept(self)
    }

    fn transform_in_line(&mut self, nodes: &[InLineNode]) -> Vec<HtmlNode> {
        nodes.iter().map(|n| n.accept(self)).collect()
    }
}

impl NodeVisitor<HtmlNode> for HtmlTransformer {
    fn visit_text(&mut self, text: &str) -> HtmlNode {
        HtmlNode::Text(text.to_string())
    }

    fn visit_emphasis(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("em"),
            attrs: vec![],
            children: self.transform_in_line(content),
        }
    }

    fn visit_strong(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("strong"),
            attrs: vec![],
            children: self.transform_in_line(content),
        }
    }

    fn visit_strikethrough(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("del"),
            attrs: vec![],
            children: self.transform_in_line(content),
        }
    }

    fn visit_heading(&mut self, level: &usize, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: format!("h{}", level),
            attrs: vec![],
            children: self.transform_in_line(content),
        }
    }

    fn visit_paragraph(&mut self, content: &[InLineNode]) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("p"),
            attrs: vec![],
            children: self.transform_in_line(content),
        }
    }
}

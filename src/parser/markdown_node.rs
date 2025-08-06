use crate::parser::{in_line_node::InLineNode, node::Node};

#[derive(Debug, Clone)]
pub enum MarkdownNode {
    Heading {
        level: usize,
        content: Vec<InLineNode>,
    },
    Paragraph(Vec<InLineNode>),
}

impl Node for MarkdownNode {
    fn accept<T>(&self, visitor: &mut dyn super::visitor::NodeVisitor<T>) -> T {
        match self {
            MarkdownNode::Heading { level, content } => visitor.visit_heading(level, content),
            MarkdownNode::Paragraph(content) => visitor.visit_paragraph(content),
        }
    }
}

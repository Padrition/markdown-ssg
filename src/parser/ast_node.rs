use crate::parser::{in_line_node::InLineNode, markdown_node::MarkdownNode};

#[derive(Debug, Clone)]
pub enum AstNode {
    Block(MarkdownNode),
    InLineNode(InLineNode),
}

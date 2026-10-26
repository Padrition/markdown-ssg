use crate::parser::{markdown_node::MarkdownNode, node::Node};

#[derive(Debug, Clone, PartialEq)]
pub enum InLineNode {
    BreakLine,
    Text(String),
    Code(Vec<InLineNode>),
    Emphasis(Vec<InLineNode>),
    Strong(Vec<InLineNode>),
    Strikethrough(Vec<InLineNode>),
    Link {
        content: Vec<InLineNode>,
        dest: String,
    },
    UnorderedListElement {
        content: Vec<InLineNode>,
        nested_list: Option<Box<MarkdownNode>>,
    },
}

impl Node for InLineNode {
    fn accept<T>(&self, visitor: &mut dyn super::visitor::NodeVisitor<T>) -> T {
        match self {
            InLineNode::Text(text) => visitor.visit_text(text),
            InLineNode::Emphasis(content) => visitor.visit_emphasis(content),
            InLineNode::Strong(content) => visitor.visit_strong(content),
            InLineNode::Strikethrough(content) => visitor.visit_strikethrough(content),
            InLineNode::Link { content, dest } => visitor.visit_link(content, dest),
            InLineNode::BreakLine => visitor.visit_break_line(),
            InLineNode::Code(content) => visitor.visit_in_line_code(content),
            InLineNode::UnorderedListElement {
                content,
                nested_list,
            } => visitor.visit_unordered_list_element(content, nested_list),
        }
    }
}

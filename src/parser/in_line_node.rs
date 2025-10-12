use crate::parser::node::Node;

#[derive(Debug, Clone)]
pub enum InLineNode {
    Text(String),
    Emphasis(Vec<InLineNode>),
    Strong(Vec<InLineNode>),
    Strikethrough(Vec<InLineNode>),
}

impl Node for InLineNode {
    fn accept<T>(&self, visitor: &mut dyn super::visitor::NodeVisitor<T>) -> T {
        match self {
            InLineNode::Text(text) => visitor.visit_text(text),
            InLineNode::Emphasis(content) => visitor.visit_emphasis(content),
            InLineNode::Strong(content) => visitor.visit_strong(content),
            InLineNode::Strikethrough(content) => visitor.visit_strikethrough(content),
        }
    }
}

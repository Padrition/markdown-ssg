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
            InLineNode::Text(_) => visitor.visit_text(self),
            InLineNode::Emphasis(_) => visitor.visit_emphasis(self),
            InLineNode::Strong(_) => visitor.visit_strong(self),
            InLineNode::Strikethrough(_) => visitor.visit_strikethrough(self),
        }
    }
}

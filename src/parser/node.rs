use crate::parser::visitor::NodeVisitor;

pub trait Node {
    fn accept<T>(&self, visitor: &mut dyn NodeVisitor<T>) -> T;
}

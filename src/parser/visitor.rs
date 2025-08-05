use crate::parser::{in_line_node::InLineNode, markdown_node::MarkdownNode};

pub trait NodeVisitor<T> {
    //In line nodes
    fn visit_text(&mut self, node: &InLineNode) -> T;
    fn visit_emphasis(&mut self, node: &InLineNode) -> T;
    fn visit_strong(&mut self, node: &InLineNode) -> T;
    fn visit_strikethrough(&mut self, node: &InLineNode) -> T;
    //Markdown nodes
    fn visit_heading(&mut self, node: &MarkdownNode) -> T;
    fn visit_paragraph(&mut self, node: &MarkdownNode) -> T;
}

pub struct AstPrinter;

impl NodeVisitor<String> for AstPrinter {
    fn visit_text(&mut self, node: &InLineNode) -> String {
        todo!()
    }

    fn visit_emphasis(&mut self, node: &InLineNode) -> String {
        todo!()
    }

    fn visit_strong(&mut self, node: &InLineNode) -> String {
        todo!()
    }

    fn visit_strikethrough(&mut self, node: &InLineNode) -> String {
        todo!()
    }

    fn visit_heading(&mut self, node: &MarkdownNode) -> String {
        todo!()
    }

    fn visit_paragraph(&mut self, node: &MarkdownNode) -> String {
        todo!()
    }
}

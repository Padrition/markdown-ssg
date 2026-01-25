use crate::parser::in_line_node::InLineNode;

pub trait NodeVisitor<T> {
    //In line nodes
    fn visit_text(&mut self, text: &str) -> T;
    fn visit_emphasis(&mut self, content: &[InLineNode]) -> T;
    fn visit_strong(&mut self, content: &[InLineNode]) -> T;
    fn visit_strikethrough(&mut self, content: &[InLineNode]) -> T;
    fn visit_link(&mut self, content: &[InLineNode], dest: &str) -> T;
    fn visit_break_line(&mut self) -> T;
    //Markdown nodes
    fn visit_heading(&mut self, level: &usize, content: &[InLineNode]) -> T;
    fn visit_paragraph(&mut self, content: &[InLineNode]) -> T;
}

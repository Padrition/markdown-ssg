use crate::parser::in_line_node::InLineNode;
use crate::parser::markdown_node::MarkdownNode;

pub trait NodeVisitor<T> {
    //In line nodes
    fn visit_text(&mut self, text: &str) -> T;
    fn visit_emphasis(&mut self, content: &[InLineNode]) -> T;
    fn visit_strong(&mut self, content: &[InLineNode]) -> T;
    fn visit_strikethrough(&mut self, content: &[InLineNode]) -> T;
    fn visit_link(&mut self, content: &[InLineNode], dest: &str) -> T;
    fn visit_break_line(&mut self) -> T;
    fn visit_in_line_code(&mut self, content: &[InLineNode]) -> T;
    fn visit_unordered_list_element(
        &mut self,
        content: &[InLineNode],
        nested_list: &Option<Box<MarkdownNode>>,
    ) -> T;

    //Markdown nodes
    fn visit_heading(&mut self, level: usize, content: &[InLineNode]) -> T;
    fn visit_paragraph(&mut self, content: &[InLineNode]) -> T;
    fn visit_horizontal_rule(&mut self) -> T;
    fn visit_markdown_code(&mut self, language: &str, content: &[InLineNode]) -> T;
    fn visit_unordered_list(&mut self, content: &[InLineNode]) -> T;
}

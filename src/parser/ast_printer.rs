use crate::parser::{
    in_line_node::InLineNode, markdown_node::MarkdownNode, node::Node, visitor::NodeVisitor,
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, node: &MarkdownNode) -> String {
        node.accept(self)
    }

    fn print_in_line(&mut self, nodes: &[InLineNode]) -> String {
        nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn parenthesize(&mut self, name: &str, children: &[InLineNode]) -> String {
        format!("({} {})", name, self.print_in_line(children))
    }
}

impl NodeVisitor<String> for AstPrinter {
    fn visit_text(&mut self, text: &str) -> String {
        format!("\"{}\"", text)
    }

    fn visit_emphasis(&mut self, content: &[InLineNode]) -> String {
        self.parenthesize("em", content)
    }

    fn visit_strong(&mut self, content: &[InLineNode]) -> String {
        self.parenthesize("strong", content)
    }

    fn visit_strikethrough(&mut self, content: &[InLineNode]) -> String {
        self.parenthesize("strike", content)
    }

    fn visit_heading(&mut self, level: &usize, content: &[InLineNode]) -> String {
        format!("(heading level={} {})", level, self.print_in_line(content))
    }

    fn visit_paragraph(&mut self, content: &[InLineNode]) -> String {
        format!("(paragraph {})", self.print_in_line(content))
    }
}

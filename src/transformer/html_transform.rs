use crate::transformer::html_node::{Display, HtmlNode};

pub struct HtmlTransformer;

impl HtmlTransformer {
    pub fn transform(&self, html_node: &HtmlNode) -> String {
        self.inner_transform(html_node, 0)
    }

    fn inner_transform(&self, html_node: &HtmlNode, level: usize) -> String {
        match html_node {
            HtmlNode::Element {
                tag,
                attrs,
                children,
                display,
            } => {
                let indent = match display {
                    Display::Inline => String::new(),
                    Display::Block => {
                        let newline = if level > 0 { "\n" } else { "" };
                        format!("{}{}", newline, &"\t".repeat(level))
                    }
                };
                let child_indent = match display {
                    Display::Inline => "",
                    Display::Block => "\t",
                };
                let trailing = match display {
                    Display::Inline => "",
                    Display::Block => "\n",
                };

                let inner: String = children
                    .iter()
                    .map(|c| self.inner_transform(c, level + 1))
                    .collect();

                format!("{indent}<{tag}>{indent}{child_indent}{inner}{indent}</{tag}>{trailing}")
            }
            HtmlNode::Text(text) => text.clone(),
        }
    }
}

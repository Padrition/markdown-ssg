use crate::transformer::html_node::HtmlNode;

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
                    crate::transformer::html_node::Display::Inline => "",
                    crate::transformer::html_node::Display::Block => &"\t".repeat(level),
                };
                let trailing = match display {
                    crate::transformer::html_node::Display::Inline => "",
                    crate::transformer::html_node::Display::Block => "\n",
                };

                let inner: String = children
                    .iter()
                    .map(|c| self.inner_transform(c, level + 1))
                    .collect();

                let opening = format!("{indent}<{tag}>");
                let closing = format!("{indent}{inner}</{tag}>");
                format!("{opening}{closing}{trailing}")
            }
            HtmlNode::Text(text) => text.clone(),
        }
    }
}

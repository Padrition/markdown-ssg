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
                let child_indent = "\t".repeat(level + 1);
                let base_indent = "\t".repeat(level);

                let attributes = if let Some(attributes) = attrs {
                    let attr_str = attributes
                        .iter()
                        .map(|t| format!("{}=\"{}\"", t.0, t.1))
                        .collect::<Vec<String>>()
                        .join(" ");

                    format!(" {}", attr_str)
                } else {
                    String::new()
                };

                let inner: String = children
                    .iter()
                    .map(|c| self.inner_transform(c, level + 1))
                    .collect();

                return match display {
                    Display::Inline => {
                        format!("<{tag}{attributes}>{inner}</{tag}>")
                    }
                    Display::Block => {
                        format!(
                            "\n{base_indent}<{tag}{attributes}>\n{child_indent}{inner}\n{base_indent}</{tag}>\n"
                        )
                    }
                    Display::Void => {
                        format!("<{tag}{attributes} />\n{base_indent}")
                    }
                };
            }
            HtmlNode::Text(text) => format!("{text}"),
        }
    }
}

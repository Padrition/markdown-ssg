use crate::transformer::html_node::{Display, HtmlNode};

pub struct HtmlTransformer;

impl HtmlTransformer {
    pub fn transform(html_node: &HtmlNode) -> String {
        HtmlTransformer::inner_transform(html_node, 0)
    }

    fn inner_transform(html_node: &HtmlNode, level: usize) -> String {
        match html_node {
            HtmlNode::Element {
                tag,
                attrs,
                children,
                display,
            } => {
                let base_indent = "\t".repeat(level);

                let attributes = if let Some(attributes) = attrs {
                    let attr_str = attributes
                        .iter()
                        .map(|t| format!("{}=\"{}\"", t.0, t.1))
                        .collect::<Vec<String>>()
                        .join(" ");

                    format!(" {attr_str}")
                } else {
                    String::new()
                };

                let next_level = match tag.as_str() {
                    "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => level,
                    _ => level + 1,
                };

                let inner: String = children
                    .iter()
                    .map(|c| HtmlTransformer::inner_transform(c, next_level))
                    .collect();

                match display {
                    Display::Inline => {
                        format!("<{tag}{attributes}>{inner}</{tag}>")
                    }
                    Display::Block => match tag.as_str() {
                        "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "pre" | "code" => {
                            format!("{base_indent}<{tag}{attributes}>{inner}</{tag}>\n")
                        }
                        _ => {
                            format!(
                                "{base_indent}<{tag}{attributes}>\n{inner}{base_indent}</{tag}>\n"
                            )
                        }
                    },
                    Display::Void => {
                        format!("<{tag}{attributes} />")
                    }
                }
            }
            HtmlNode::Text(text) => text.to_string(),
        }
    }
}

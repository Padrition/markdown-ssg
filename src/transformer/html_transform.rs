use crate::transformer::html_node::HtmlNode;

pub struct HtmlTransformer;

impl HtmlTransformer {
    pub fn transform(&self, html_node: &HtmlNode) -> String {
        match html_node {
            HtmlNode::Element {
                tag,
                attrs,
                children,
            } => {
                let inner: String = children.iter().map(|c| self.transform(c)).collect();
                format!("<{tag}> {inner} </{tag}>")
            }
            HtmlNode::Text(text) => text.clone(),
        }
    }
}

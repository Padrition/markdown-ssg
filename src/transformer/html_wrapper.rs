use crate::transformer::html_node::{Display, HtmlNode};

pub struct HtmlWrapper;

impl HtmlWrapper {
    pub fn wrap(hast: Vec<HtmlNode>) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("html"),
            attrs: vec![],
            display: Display::Block,
            children: vec![
                HtmlNode::Element {
                    tag: String::from("head"),
                    attrs: vec![],
                    display: Display::Block,
                    children: vec![HtmlNode::Element {
                        tag: String::from("title"),
                        attrs: vec![],
                        display: Display::Inline,
                        children: vec![HtmlNode::Text(String::from("My page"))],
                    }],
                },
                HtmlNode::Element {
                    tag: String::from("body"),
                    attrs: vec![],
                    display: Display::Block,
                    children: hast,
                },
            ],
        }
    }
}

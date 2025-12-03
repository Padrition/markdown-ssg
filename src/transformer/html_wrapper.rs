use crate::transformer::html_node::{Display, HtmlNode};

pub struct HtmlWrapper;

impl HtmlWrapper {
    pub fn wrap(hast: Vec<HtmlNode>) -> HtmlNode {
        HtmlNode::Element {
            tag: String::from("html"),
            attrs: None,
            display: Display::Block,
            children: vec![
                HtmlNode::Element {
                    tag: String::from("head"),
                    attrs: None,
                    display: Display::Block,
                    children: vec![HtmlNode::Element {
                        tag: String::from("title"),
                        attrs: None,
                        display: Display::Inline,
                        children: vec![HtmlNode::Text(HtmlWrapper::create_title(hast.first()))],
                    }],
                },
                HtmlNode::Element {
                    tag: String::from("body"),
                    attrs: None,
                    display: Display::Block,
                    children: hast,
                },
            ],
        }
    }

    fn create_title(node: Option<&HtmlNode>) -> String {
        let default_title = String::from("My page");

        let Some(HtmlNode::Element { tag, children, .. }) = node else {
            return default_title;
        };

        if tag != "h1" {
            return default_title;
        }

        let mut title = String::new();

        HtmlWrapper::collect_title(children, &mut title);

        title
    }

    fn collect_title(nodes: &Vec<HtmlNode>, out: &mut String) {
        for child in nodes {
            match child {
                HtmlNode::Element { children, .. } => HtmlWrapper::collect_title(children, out),
                HtmlNode::Text(text) => out.push_str(text),
            }
        }
    }
}

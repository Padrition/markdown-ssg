use std::path::Path;

use clap::builder::OsStr;

use crate::transformer::html_node::{Display, HtmlNode};

pub struct HtmlWrapper;

impl HtmlWrapper {
    pub fn create_index<P: AsRef<Path>>(paths: &[P]) -> HtmlNode {
        let links: Vec<HtmlNode> = paths
            .iter()
            .map(|p| {
                let path = p.as_ref();

                let href = path
                    .to_owned()
                    .with_extension("html")
                    .file_name()
                    .unwrap_or(&OsStr::from("index.html"))
                    .to_string_lossy()
                    .to_string();

                let name = path
                    .to_owned()
                    .with_extension("")
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&href)
                    .to_string();

                HtmlNode::Element {
                    tag: String::from("a"),
                    attrs: Some(vec![("href".to_owned(), href)]),
                    children: vec![HtmlNode::Text(name)],
                    display: Display::Block,
                }
            })
            .collect();

        HtmlWrapper::wrap(links)
    }

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

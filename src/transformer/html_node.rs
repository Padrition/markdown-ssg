#[derive(Debug, Clone, PartialEq)]
pub enum Display {
    Inline,
    Block,
    Void,
}

#[derive(Debug, Clone)]
pub enum HtmlNode {
    Element {
        tag: String,
        attrs: Option<Vec<(String, String)>>,
        children: Vec<HtmlNode>,
        display: Display,
    },
    Text(String),
}

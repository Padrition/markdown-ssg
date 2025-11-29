#[derive(Debug, Clone)]
pub enum Display {
    Inline,
    Block,
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

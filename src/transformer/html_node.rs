#[derive(Debug, Clone)]
pub enum HtmlNode {
    Element {
        tag: String,
        attrs: Vec<(String, String)>,
        children: Vec<HtmlNode>,
    },
    Text(String),
}

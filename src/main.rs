mod markdown;

use markdown::{parse_heading,parse_paragraph};

const HEADING: &str = "#Heading";
const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
fn main() {
    println!("{}", parse_heading(HEADING).unwrap_or("no".to_string()));
    println!("{}", parse_paragraph(PARAGRAPH).unwrap_or("no".to_string()));
}

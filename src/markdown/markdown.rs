pub fn parse_paragraph(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        return None;
    }

    Some(format!("<p>{s}</p>"))
}

pub fn parse_heading(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        return None;
    }

    let mut parts = s.splitn(2, " ");
    let level = parts.next()?;
    let rest = parts.next().unwrap_or("").trim();

    if !level.starts_with("#") {
        return None;
    }

    match level.len() {
        6.. => Some(format!("<h6>{rest}</h6>")),
        n => Some(format!("<h{n}>{rest}</h{n}>")),
    }
}

#[cfg(test)]
mod markdown_tests {
    use super::*;

    #[test]
    fn test_parse_paragraph_empty() {
        let parsed_paragraph = parse_paragraph("");
        assert_eq!(parsed_paragraph, None);
    }

    #[test]
    fn test_parse_paragraph() {
        let text = "Lorem ipsum dolor sit amet, 
            consectetuer adipiscing elit. 
            Aliquam erat volutpat. Nullam
            sapien sem, ornare ac, nonummy
            non, lobortis a enim.";

        let parsed_paragraph = parse_paragraph(text);
        let awaited_output = format!("<p>{text}</p>");
        assert_eq!(parsed_paragraph, Some(awaited_output));
    }

    #[test]
    fn test_parse_heading_empty() {
        let parsed_heading = parse_heading("");
        assert_eq!(parsed_heading, None);
    }

    #[test]
    fn test_parse_heading_not_a_heading() {
        let parsed_heading = parse_heading("not a heading");
        assert_eq!(parsed_heading, None)
    }

    #[test]
    fn test_parse_heading() {
        for level in 1..6 {
            let hashes = "#".repeat(level);
            let text = "Heading";
            let input = format!("{hashes} {text}");
            let expected = format!("<h{level}>{text}</h{level}>");
            let parsed_heading = parse_heading(&input);
            assert_eq!(parsed_heading, Some(expected));
        }
    }

    #[test]
    fn test_parse_heading_bigger() {
        let hashes = "#".repeat(7);
        let text = "Heading";
        let input = format!("{hashes} {text}");
        let expected = format!("<h6>{text}</h6>");
        let parsed_heading = parse_heading(&input);
        assert_eq!(parsed_heading, Some(expected));
    }
}

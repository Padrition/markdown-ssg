use std::os::linux::raw::stat;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidHeadingSyntax,
}

pub fn parse_paragraph(s: &str) -> Result<String, ParseError> {
    if s.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }

    Ok(format!("<p>{s}</p>"))
}

pub fn parse_heading(s: &str) -> Result<String, ParseError> {
    if s.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut parts = s.splitn(2, " ");
    let level = parts.next().ok_or(ParseError::InvalidHeadingSyntax)?;
    let rest = parts.next().unwrap_or("").trim();

    if !level.starts_with("#") {
        return Err(ParseError::InvalidHeadingSyntax);
    }

    match level.len() {
        6.. => Ok(format!("<h6>{rest}</h6>")),
        n => Ok(format!("<h{n}>{rest}</h{n}>")),
    }
}

pub fn parse_bold(s: &str) -> Result<String, ParseError>{
    if s.trim().is_empty(){
        return Err(ParseError::EmptyInput);
    }

    let mut output = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() && ((chars[i] == '*' && chars[i + 1] == '*') || (chars[i] == '_' && chars[i + 1] == '_')){
            let marker = chars[i];
            i += 2;

            let start = i;

            while i + 1 < chars.len() && !(chars[i] == marker && chars[i + 1] == marker) {
                i += 1;
            }

            if i + 1 < chars.len() {
                let bold_text: String = chars[start..i].iter().collect();
                output.push_str(&format!("<b>{bold_text}</b>"));
                i += 2;
            }else {
                output.push(marker);
                output.push(marker);
                output.extend(&chars[start..]);
                break;
            }
        }else{
            output.push(chars[i]);
            i += 1;
        }
        
    }

    Ok(output)
}


#[cfg(test)]
mod markdown_tests {
    use super::*;

    #[test]
    fn test_parse_paragraph_empty() {
        let parsed_paragraph = parse_paragraph("");
        assert_eq!(parsed_paragraph, Err(ParseError::EmptyInput));
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
        assert_eq!(parsed_paragraph, Ok(awaited_output));
    }

    #[test]
    fn test_parse_heading_empty() {
        let parsed_heading = parse_heading("");
        assert_eq!(parsed_heading, Err(ParseError::EmptyInput));
    }

    #[test]
    fn test_parse_heading_not_a_heading() {
        let parsed_heading = parse_heading("not a heading");
        assert_eq!(parsed_heading, Err(ParseError::InvalidHeadingSyntax))
    }

    #[test]
    fn test_parse_heading() {
        for level in 1..6 {
            let hashes = "#".repeat(level);
            let text = "Heading";
            let input = format!("{hashes} {text}");
            let expected = format!("<h{level}>{text}</h{level}>");
            let parsed_heading = parse_heading(&input);
            assert_eq!(parsed_heading, Ok(expected));
        }
    }

    #[test]
    fn test_parse_heading_bigger() {
        let hashes = "#".repeat(7);
        let text = "Heading";
        let input = format!("{hashes} {text}");
        let expected = format!("<h6>{text}</h6>");
        let parsed_heading = parse_heading(&input);
        assert_eq!(parsed_heading, Ok(expected));
    }

    #[test]
    fn test_parse_bold_empty(){
        let parsed_bold = parse_bold("");
        assert_eq!(parsed_bold, Err(ParseError::EmptyInput));
    }

    #[test]
    fn test_parse_bold(){
        let bold = "**bold text**";
        let expected = format!("<b>bold text</b>");
        let parsed_bold = parse_bold(bold);
        assert_eq!(parsed_bold, Ok(expected));
    }
}

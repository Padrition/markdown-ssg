pub fn parse_paragraph(s: &str)->Option<String>{
    if s.trim().is_empty(){
        return None;
    }

    Some(format!("<p>{s}</p>"))
}

pub fn parse_heading(s: &str)->Option<String>{
    if s.trim().is_empty() {
        return None;
    }

    let mut parts = s.splitn(2, " ");
    let level = parts.next()?;
    let rest = parts.next().unwrap_or("").trim();

    if !level.starts_with("#"){
        return None
    }

    match level.len() {
        6.. => {
            Some(format!("<h6>{rest}</h6>"))
        },
        n => {
            Some(format!("<h{n}>{rest}</h{n}>"))
        }
    }
}
use regex::Regex;

// <p></p> to "\n text \n", seems as well that there might be just <p> defined without closing tag
// <a href="link" rel="no-follow">link</a> to single link
// remove other tags for now like <i> </i>

pub enum FormatType {
    Paragraph,
    Link,
    Other,
}

fn replace_paragraphs(s:String) -> String {
    let mut r = s.replace("<p>", "\n");
    r = r.replace("</p>", "");
    r
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_paragraphs() {
        let mut s:String = format!("aaaaaa zzz zzaaa <p> </p>");
        s = replace_paragraphs(s);
        assert!(!s.contains("<p>"));
        assert!(!s.contains("</p>"));

    }
}
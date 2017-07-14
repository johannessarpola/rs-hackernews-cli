use regex::Regex;

// <p></p> to "\n text \n", seems as well that there might be just <p> defined without closing tag
// <a href="link" rel="no-follow">link</a> to single link
// remove other tags for now like <i> </i>

pub enum FormatType {
    Paragraph,
    Link,
    Other,
}

fn replace_paragraphs(s: String) -> String {
    // todo split
    let mut r = replace_paragraphs_opening_tags(&s);
    r = replace_paragraphs_closing_tags(&r);
    r
}

fn replace_paragraphs_closing_tags(s: &str) -> String {
    let closing_tags_with_whitespace = Regex::new(r"(\s*</p>\s*)").unwrap();
    closing_tags_with_whitespace.replace_all(s, "").into_owned()
}

fn replace_paragraphs_opening_tags(s: &str) -> String {
    let opening_tags_with_whitespace = Regex::new(r"(\s*<p>\s*)").unwrap();
    opening_tags_with_whitespace.replace_all(s, "\n").into_owned()
}

fn remove_double_links(s: String) -> String {
    "".to_owned()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_paragraphs() {
        let mut s: String = format!("aaaaaa zzz zzaaa <p> </p>");
        s = replace_paragraphs(s);
        assert!(!s.contains("<p>"));
        assert!(!s.contains("</p>"));
        assert_eq!("aaaaaa zzz zzaaa", s);

    }
}
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
    let re = Regex::new(r"( *</p> *)").unwrap();
    re.replace_all(s, "").into_owned()
}

fn replace_paragraphs_opening_tags(s: &str) -> String {
    let re = Regex::new(r"( *<p> *)").unwrap();
    re.replace_all(s, "\n").into_owned()
}

fn replace_link_opening_tags(s: &str) -> String {
    let re = Regex::new(r"(<a+.*href=\x22.\x22*\s*?.*?\s*?> *)").unwrap();
    re.replace_all(s, "").into_owned()
}

fn replace_link_closing_tags(s: &str) -> String {
    let re = Regex::new(r"( *</a> *)").unwrap();
    re.replace_all(s, "").into_owned()
}

fn replace_links(s: String) -> String {
    let mut r = replace_link_opening_tags(&s);
    r = replace_link_closing_tags(&r);
    r
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_paragraphs() {
        let mut s: String = format!("aaaaaa zzz zzaaa <p> </p>");
        s = replace_paragraphs(s);
        assert_eq!("aaaaaa zzz zzaaa\n", s);

        s = format!("<p> para</p> ");
        s = replace_paragraphs(s);
        assert_eq!("\npara", s);
    }


    #[test]
    fn test_replace_links() {
        let mut s: String = format!("this is <a href=\"http\"://www.link.com rel=\"nofollow\"> \
                                     http://www.link.com </a>");
        s = replace_links(s);
        assert_eq!("this is http://www.link.com", s);
    }


}
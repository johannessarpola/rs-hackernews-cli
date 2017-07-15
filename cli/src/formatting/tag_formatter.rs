use regex::Regex;
use super::formatter::Formatter;

pub struct TagFormatter;

impl Formatter for TagFormatter {
    fn format(s: &str) -> String {
        String::from("s")
    }
}

impl TagFormatter {
    pub fn format_paragraphs(&self, s: &str) -> String {
        // todo split
        let mut r = self.replace_paragraphs_opening_tags(&s, "\n");
        r = self.replace_paragraphs_closing_tags(&r, "");
        r
    }

    pub fn format_links(&self, s: &str) -> String {
        let mut r = self.replace_link_opening_tags(s, "");
        r = self.replace_link_closing_tags(&r, "");
        r
    }

    pub fn format_tags(&self, s: &str) -> String {
        self.replace_tags(s, "")
    }

    fn replace_paragraphs_closing_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"( *</p> *)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_paragraphs_opening_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"( *<p> *)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_link_opening_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"(<a+.*href=\x22.\x22*\s*?.*?\s*?> *)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_link_closing_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"( *</a> *)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_tags(&self, s: &str, replacement: &str) -> String {
        // will just replace all tags and should be called after other methods have run before
        let re = Regex::new(r"(</?\w+((\s+\w+(\s*=\s*(?:\x22.*?\x22|'.*?'|[\^'\x22>\s]+))?)+\s*|\s*)/?>) *").unwrap();
        re.replace_all(s, replacement).into_owned()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_paragraphs() {
        let tag_formatter = TagFormatter;
        let mut s: String = format!("aaaaaa zzz zzaaa <p> </p>");
        s = tag_formatter.format_paragraphs(&s);
        assert_eq!("aaaaaa zzz zzaaa\n", s);

        s = format!("<p> para</p> ");
        s = tag_formatter.format_paragraphs(&s);
        assert_eq!("\npara", s);
    }


    #[test]
    fn test_replace_links() {
        let tag_formatter = TagFormatter;
        let mut s: String = format!("this is <a href=\"http\"://www.link.com rel=\"nofollow\"> \
                                     http://www.link.com </a>");
        s = tag_formatter.format_links(&s);
        assert_eq!("this is http://www.link.com", s);
    }

    #[test]
    fn test_replace_tags() {
        let tag_formatter = TagFormatter;
        let mut s: String = format!("this is <a href=\"http\"://www.link.com \
                                     rel=\"nofollow\">http://www.link.com</a>");
        s = tag_formatter.format_tags(&s);
        assert_eq!("this is http://www.link.com", s);
        s = format!("<p> para </p><div>div </div> <h1>heading </h1>");
        s = tag_formatter.format_tags(&s);
        assert_eq!("para div heading ", s);
    }


}
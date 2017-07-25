use regex::Regex;
use super::formatter::FormatStr;

pub struct TagFormatter;

impl FormatStr for TagFormatter {
    fn format(&self, s: &str) -> String {
        let mut r = self.format_paragraphs(s);
        // currently no special formatting for links so just format like other tags (remove)
        r = self.format_code_tags(&r);
        self.format_tags(&r)
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

    pub fn format_code_tags(&self, s:&str) -> String {
        let mut r = self.replace_opening_code_tags(s, "\n");
        self.replace_closing_code_tags(&r, "\n") // make code blocks stand out
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
        let re = Regex::new(r"(</?\w+((\s+\w+(\s*=\s*(?:\x22.*?\x22|'.*?'|[\^'\x22>\s]+))?)+\s*|\s*)/?>)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_closing_code_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"(</code> *)").unwrap();
        re.replace_all(s, replacement).into_owned()
    }

    fn replace_opening_code_tags(&self, s: &str, replacement: &str) -> String {
        let re = Regex::new(r"( *<code>)").unwrap();
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
    fn test_replace_codetags() {
        let tag_formatter = TagFormatter;
        let mut s = format!("<code> System.out.println(\"Hello world\"); </code>");
        s = tag_formatter.format_code_tags(&s);
        assert_eq!("\n\tSystem.out.println(\"Hello world\");", s);
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
        s = format!("<p>para </p><div>div </div><h1>heading </h1>");
        s = tag_formatter.format_tags(&s);
        assert_eq!("para div heading ", s);
    }
    #[test]
    fn test_formatting_with_item() {
        use helpers::io_utils::read_file;
        use serde_json;
        use core::models::HnItem;

        let tag_formatter = TagFormatter;

        let unformatted_item:HnItem =  read_file("res/test/item-with-html.json")
            .and_then(|content| Some(serde_json::from_str(&content).unwrap()))
            .unwrap();
        let formatted_text = read_file("res/test/formatted-item-with-html.txt").unwrap();
        assert_eq!(formatted_text, tag_formatter.format(unformatted_item.text.as_ref().unwrap()));
        // ;
    }

}
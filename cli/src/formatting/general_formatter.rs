use super::formatter::FormatStr;

pub struct GeneralFormatter;

const LINE_LEN: usize = 80;

impl FormatStr for GeneralFormatter {
    fn format(&self, s: &str) -> String {
        let r = self.format_to_length(s);
        r
    }
}

impl GeneralFormatter {
    fn format_to_length(&self, s: &str) -> String {
        let r = s.to_owned();
        let words = r.split(" ");
        let mut result: String = String::new();
        let mut characters_since_newline: usize = 0;
        for word in words {
            if word != "\n" && !word.contains('\n') {
                characters_since_newline += word.len() + 1;
            } else {
                characters_since_newline = 0;
            }
            if characters_since_newline >= LINE_LEN { // Currently since the plus is before it'll 
                result.push('\n');                    // add newline if the line would be over the limit and then push the word
                characters_since_newline = 0;
            }
            result.push_str(word);
            result.push(' ');
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::btree_map::BTreeMap;

    #[test]
    fn test_line_len_formatting() {
        let f = GeneralFormatter;
        let s: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas maximus \
                       eleifend nibh, eget fringilla augue mattis ac. Nunc rhoncus dolor lorem, \
                       sed gravida mi tempus a. Maecenas libero nunc, mollis sit amet ex at, \
                       luctus bibendum dui. Donec dignissim, sapien nec commodo vehicula, tortor \
                       sem euismod leo, ac convallis risus.";
        let r = f.format_to_length(s);
        let mut counts = BTreeMap::new();
        for c in r.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let count = counts.get(&'\n').unwrap();
        assert_eq!(*count, (s.len() / LINE_LEN) - 1); // there's no newline at the end
        assert!(*count > 0);

    }
}

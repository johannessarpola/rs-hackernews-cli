use super::tag_formatter::TagFormatter;
use super::general_formatter::GeneralFormatter;

pub trait FormatStr {
    fn format(&self, s: &str) -> String;
}

pub struct Formatters {
    formatters: Vec<Box<FormatStr>>,
}

impl Formatters {

    pub fn new() -> Formatters {
        Formatters {
            formatters: vec!(Box::new(TagFormatter), Box::new(GeneralFormatter))
        }
    }
}
impl FormatStr for Formatters {
    fn format(&self, s: &str) -> String {
        let mut working_copy:String = self.formatters[0].format(s);
        for f in self.formatters.iter().skip(1) {
            working_copy = f.format(&working_copy);
        }
        working_copy
    }
}
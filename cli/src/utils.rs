use hyper::{Uri, StatusCode};
use slog::*;
use models::HnItem;

pub fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}
pub fn rand_string(length: i32) -> String {
    String::from("abc") // TODO Implement
}
pub fn parse_url_from_str(url_str: &str) -> Uri {
    let url_str = String::from(url_str);
    let url = url_str.parse::<Uri>().unwrap();
    url
}

pub fn comment_has_kids(item: &HnItem) -> bool {
    match item.kids {
        Some(ref kids) => true,
        None => false,
    }
}

pub fn try_to_parse_number(s: Option<&str>) -> Option<usize> {
    match s {
        Some(s) => {
            let parse_result = s.parse::<usize>();
            if parse_result.is_ok() {
                Some(parse_result.unwrap())
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn filename_for_hnitem(item: &HnItem) -> String {
    match item.title {
        Some(ref title) => return combine_strings(vec![&title, &item.by, ".html"]),
        None => {
            return combine_strings(vec![&parse_url_from_str(&item.url.as_ref().unwrap())
                                            .path()
                                            .replace("/", "_"),
                                        ".html"])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_url_from_str_test() {
        let url = parse_url_from_str("http://www.google.fi");
        assert_eq!("http", url.scheme().unwrap());
        assert_eq!("www.google.fi", url.authority().unwrap());
    }

    #[test]
    fn combine_strings_test() {
        let a = "Abc";
        let b = "Abc";
        let mut vec = Vec::new();
        vec.push(a);
        vec.push(b);
        assert_eq!("AbcAbc", combine_strings(vec));
        assert!(a.len() > 1);
        assert!(b.len() > 1);
    }
}
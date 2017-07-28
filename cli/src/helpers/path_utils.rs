use hyper::Uri;
use core::models::HnItem;
use url::{Url, Host};

pub fn parse_url_from_str(url_str: &str) -> Uri {
    let url_str = String::from(url_str);
    let url = url_str.parse::<Uri>().unwrap();
    url
}

pub fn get_host_from_link(path: &str) -> Option<String> {
    let result = Url::parse(path);
    if result.is_ok() {
        let url: Url = result.unwrap();
        match url.host_str() {
            Some(host) => Some(String::from(host)),
            None => None,
        }
    } else {
        None
    }
}

pub fn generate_filename_for_hnitem(item: &HnItem) -> String {
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

fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
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
    fn get_host_from_link_test() {
        let mut s = "http://www.google.fi";
        let mut os = get_host_from_link(s);
        assert_eq!("www.google.fi", os.unwrap());

        s = "abcabc";
        os = get_host_from_link(s);
        assert!(os.is_none());
    }
}
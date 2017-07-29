use core::models::HnItem;
use url::{Url};
use std::path::Path;

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

pub fn get_filesystem_safe_url_as_string(path: &str) -> Option<String> {
    let url_result = Url::parse(path);
    if url_result.is_ok() {
        let url: Url = url_result.unwrap();
        let url_str = format!("{}{}", url.host_str().unwrap_or("could_not_parse_host"), url.path());
        let path_str = Path::new(&url_str).to_string_lossy().into_owned();
        Some(format!("{}.html", path_str))
    } else {
        None
    }

}

pub fn generate_filename_for_hnitem(item: &HnItem) -> String {
    match item.title {
        Some(ref title) => return combine_strings(vec![&title, &item.by, ".html"]),
        None => {
            let fname = get_filesystem_safe_url_as_string(&item.url.as_ref().unwrap())
                .unwrap_or(String::from("could_not_create_filename.html"));
            fname
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
    fn get_host_from_link_test() {
        let mut s = "http://www.google.fi";
        let mut os = get_host_from_link(s);
        assert_eq!("www.google.fi", os.unwrap());

        s = "abcabc";
        os = get_host_from_link(s);
        assert!(os.is_none());
    }
    #[test]
    fn get_filesystem_safe_url_as_string_test() {
        let s = "http://www.google.fi/search/";
        let fs = get_filesystem_safe_url_as_string(s);
        assert_eq!("www.google.fi/search/.html", fs.unwrap());
    }
}
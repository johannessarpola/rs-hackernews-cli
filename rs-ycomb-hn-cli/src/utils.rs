use hyper::{Uri, StatusCode};
use slog::*;

pub fn log_response_status(logger: &Logger, url: &str, status: &StatusCode) {
    info!(logger,
          format!("Request to {} finished with status {}", url, status));
}

pub fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}

pub fn parse_url_from_str(url_str: &str) -> Uri {
    let url_str = String::from(url_str);
    let url = url_str.parse::<Uri>().unwrap();
    url
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
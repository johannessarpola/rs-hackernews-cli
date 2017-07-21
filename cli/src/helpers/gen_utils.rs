use hyper::{Uri};
use core::models::HnItem;

pub fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}

pub fn comment_has_kids(item: &HnItem) -> bool {
    match item.kids {
        Some(_) => true,
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

#[cfg(test)]
mod tests {
    use super::*;

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
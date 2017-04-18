use models::*;

pub fn print_headline_with_author(item: &HnItem, index: &i32) {
    let s = create_headline_with_author(item, index).unwrap(); // Not handling errs
    println!("{}", s);
}

fn create_headline_with_author(item: &HnItem, index: &i32) -> Result<String, String> {
    match item.title {
        Some(_) => {
            let s = format!("[{:3}] {:80} ~ {}",
                            index,
                            item.title.as_ref().unwrap(),
                            item.by);
            Ok(s)

        }
        None => Err(String::from("Not headline")),
    }
}

pub fn could_not_get_any_commments_for_item(item: &HnItem) {
    println!("Could not get comments for item with id {}", item.id)
}

pub fn print_filename_of_loaded_page(filen: &str, title: &str) {
    println!("{} {} {} {}", "Downloaded page", title, "into file", filen);
}
pub fn could_not_load_page(title: &str) {
    println!("Could not download to file with title {}", title);
}

pub fn print_comments(item: &HnItem, vec: &Vec<HnItem>) {
    // FIXME No error handling
    println!("Comments for item id {} with title {}",
             &item.id,
             item.title.as_ref().unwrap());
    let mut comment_index = 0;
    for comment in vec {
        comment_index += 1; // Should be from main?
        let res = create_comment_row(&comment_index, &comment);
        if (res.is_some()) {
            println!("{}", res.unwrap());
        }
    }
}
fn create_comment_row(index: &i32, item: &HnItem) -> Option<String> {
    match item.text {
        Some(ref text) => {
            let s = format!("[{:3}] {:80} ~{}", index, text, &item.by); // TODO This needs to handle unicode characters to utf8 or something similar (snap&#x27;s -> snap's)
            Some(s)
        }
        None => None,
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_headline_with_author_test() {
        use std::fs::File;
        use std::io::prelude::*;
        use serde_json;
        let mut contents = String::new();
        File::open("res/test/item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let index = 1;
        let s: String = create_headline_with_author(&deserialized, &index).unwrap();
        assert!(s.len() != 0);
        assert!(s.contains("1"));
        assert!(s.contains("dhouston"));
        assert!(s.contains("My YC app: Dropbox - Throw away your USB drive"));
        assert!(deserialized.by.len() != 0);
        assert!(deserialized.title.unwrap().len() != 0);
    }
}
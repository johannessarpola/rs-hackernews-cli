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
        let s:String = create_headline_with_author(&deserialized, &index).unwrap();
        assert!(s.len() != 0);
        assert!(s.contains("1"));
        assert!(s.contains("dhouston"));
        assert!(s.contains("My YC app: Dropbox - Throw away your USB drive"));
        assert!(deserialized.by.len() != 0);
        assert!(deserialized.title.unwrap().len() != 0);
    }
}
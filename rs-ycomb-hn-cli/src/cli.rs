use models::*;
use std::iter::repeat;

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

pub fn print_comments(items: &Vec<HnItem>) {
    println!("Comments: \n");
    for item in items {
        println!("{}", create_comment_string(item, &1).unwrap()); // todo depth and error handling
    }
}

fn create_comment_string(item:&HnItem, depth:&usize) -> Result<String, String> {
    let depthStr = repeat("-").take(*depth).collect::<String>();
    if(item.text.is_some()) {
        return Ok(format!("{} {} ~ {} on {}", depthStr, item.text.as_ref().unwrap(), &item.by, &item.time))
    }
    else {
        return Err(String::from("Could not get text for comment"))
    }
}

pub fn no_comments_for(numb:&usize) {
    println!("No comments for [{}]", numb);
}

pub fn print_filename_of_loaded_page(filen:&str, title:&str){
    println!("{} {} {} {}", "Downloaded page", title, "into file", filen);
}
pub fn could_not_load_page(title:&str){
    println!("Could not download to file with title {}", title);
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

    #[test]
    fn create_comment_string_test() {
        use std::fs::File;
        use std::io::prelude::*;
        use serde_json;
        let mut contents = String::new();
        File::open("res/test/children-item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let commentStr = create_comment_string(&deserialized, &2).unwrap();
        assert!(commentStr.contains("is not a valid concern. Unless you are planning"));
        assert!(commentStr.contains("cholantesh"));
        assert!(commentStr.contains("-- "));
        
    }
}
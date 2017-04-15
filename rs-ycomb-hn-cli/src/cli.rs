use models::*;

pub fn print_headline_with_author(item: &HnItem, index: &i32) {
    let s = create_headline_with_author(item, index).unwrap(); // Not handling errs
    println!("{}", s);
}

fn create_headline_with_author(item: &HnItem, index: &i32) -> Result<String, String> {
    match item.title {
        Some(ref title) => {
            let s = format!("[{:3}] {:80} ~ {}",
                            &index,
                            &item.title.as_ref().unwrap(),
                            &item.by);
            Ok(s)

        }
        None => Err(String::from("Not headline")),
    }
}
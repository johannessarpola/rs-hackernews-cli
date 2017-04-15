use models::*;

pub fn print_headline_with_author(item:&HnItem, index:&i32) {
    let s = create_headline_with_author(item, index).unwrap(); // Not handling errs
    println!("{}", s);
}

fn create_headline_with_author(item:&HnItem, index:&i32) -> Result<String,String> {
    match item.title {
        Some(ref title) => Ok(format!("[{}] {} ~{}", &index, &item.title.as_ref().unwrap(), &item.by)),
        None => Err(String::from("Not headline"))
    }
}
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Option<String>  {
    let mut contents = String::new();
    let file = File::open(path);
    if file.is_ok() {
        let result = file.unwrap().read_to_string(&mut contents); // todo do something with result?
    }
    if contents.len() > 0 {
        Some(contents)
    }
    else {
        None
    }
}

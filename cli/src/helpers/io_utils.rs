use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn read_file(path: &str) -> Option<String>  {
    let mut contents = String::new();
    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut contents));
    if contents.len() > 0 {
        Some(contents)
    }
    else {
        None
    }
}

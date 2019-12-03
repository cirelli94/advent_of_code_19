use std::fs::File;
use std::io::prelude::*;

pub fn split_whitespace_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|x| String::from(x)).collect()
}

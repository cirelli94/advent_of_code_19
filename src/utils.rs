use std::fs::File;
use std::io::prelude::*;

pub fn split_whitespace_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|x| String::from(x)).collect()
}

pub fn split_char_file(filename: &str, character: char) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();
    buffer.split(character).map(|x| String::from(x)).collect()
}

pub fn convert_strings_to_u32(strings: Vec<String>) -> Vec<u32> {
    strings.iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

pub fn convert_strings_to_usize(strings: Vec<String>) -> Vec<usize> {
    strings
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

mod tokenizer;

fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");

    println!("{:?}", tokenizer::tokenize(contents.chars().collect()));
}

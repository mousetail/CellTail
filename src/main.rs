use std::env;
use std::fs;

mod lexer;
mod tokenizer;

fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");

    let tokens = tokenizer::tokenize(contents.chars().collect());
    let lexical_tokens = lexer::lex(tokens);

    println!("{:#?}", lexical_tokens)
}

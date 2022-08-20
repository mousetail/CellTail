use std::env;
use std::fs;

mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod tokenizer;

fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");

    let tokens = tokenizer::tokenize(contents.chars().collect());
    let lexical_tokens = lexer::lex(tokens);

    println!("{:#?}", lexical_tokens);
    let structure = parser::parse(lexical_tokens);

    // let mut buff: Vec<u8> = vec![];
    // stdin()
    //     .read_to_end(&mut buff)
    //     .expect("Failed to read input file");

    interpreter::interpret(
        structure,
        env::args()
            .nth(2)
            .expect("Need 2 arguments")
            .bytes()
            .collect(),
    )
}

use std::env;
use std::fs;

mod errors;
mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod tokenizer;

fn parse_and_run_code(code: &Vec<char>) -> errors::CellTailResult<()> {
    let tokens = tokenizer::tokenize(code)?;
    let lexical_tokens = lexer::lex(tokens)?;

    // println!("{:#?}", lexical_tokens);
    let structure = parser::parse(lexical_tokens)?;

    // let mut buff: Vec<u8> = vec![];
    // stdin()
    //     .read_to_end(&mut buff)
    //     .expect("Failed to read input file");

    interpreter::run_program(structure, env::args().skip(2).collect())
}

fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let contents_chars = contents.chars().collect::<Vec<char>>();

    if let Err(b) = parse_and_run_code(&contents_chars) {
        b.print(contents_chars);
    }
}

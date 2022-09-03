#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use serde_json::to_string;
use std::env;
use std::fs;
#[cfg(target_arch = "wasm32")]
use std::panic;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod checker;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod tokenizer;

fn parse_and_run_code(code: &Vec<char>, input: Vec<String>) -> errors::CellTailResult<()> {
    let tokens = tokenizer::tokenize(code)?;
    let lexical_tokens = lexer::lex(tokens)?;
    let structure = parser::parse(lexical_tokens)?;

    checker::check_program(&structure)?;

    if structure.attributes.debug {
        println!("{:?}", structure);
    }

    interpreter::run_program(structure, input)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_parse_run_code(code: &str, input: &str) -> String {
    handle_output("bye");
    handle_error("hello");

    to_string(&parse_and_run_code(
        &code.chars().collect(),
        vec![input.to_owned()],
    ))
    .unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let contents_chars = contents.chars().collect::<Vec<char>>();

    if let Err(b) = parse_and_run_code(&contents_chars, env::args().skip(2).collect()) {
        b.print(contents_chars);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn handle_output(value: &str);
    fn handle_error(value: &str);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook))
}

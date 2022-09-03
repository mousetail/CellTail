#[cfg(target_arch = "wasm32")]
use crate::wasm_output::FunctionWriter;
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
#[cfg(target_arch = "wasm32")]
mod wasm_output;

fn parse_and_run_code<T: std::io::Write>(
    code: &Vec<char>,
    input: Vec<String>,
    output: &mut T,
) -> errors::CellTailResult<()> {
    let tokens = tokenizer::tokenize(code)?;
    let lexical_tokens = lexer::lex(tokens)?;
    let structure = parser::parse(lexical_tokens)?;

    checker::check_program(&structure)?;

    if structure.attributes.debug {
        println!("{:?}", structure);
    }

    interpreter::run_program(structure, input, output)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_parse_run_code(code: &str, input: &str) -> String {
    parse_and_run_code(
        &code.chars().collect(),
        vec![input.to_owned()],
        &mut wasm_output::FunctionWriter::create_stdout(),
    );

    "".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let filename = env::args().nth(1).expect("Expected at least one argument");
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let contents_chars = contents.chars().collect::<Vec<char>>();

    if let Err(b) = parse_and_run_code(
        &contents_chars,
        env::args().skip(2).collect(),
        &mut std::io::stdout(),
    ) {
        b.print(contents_chars);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook))
}

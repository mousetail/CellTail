use crate::tokenizer::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenGroup {
    delimiter: Option<char>,
    contents: Vec<LexerToken>,
}

#[derive(Debug)]
pub enum LexerToken {
    Group(TokenGroup),
    BasicToken(Token),
}

pub fn lex(input: Vec<Token>) -> TokenGroup {
    let mut stack: Vec<TokenGroup> = vec![TokenGroup {
        delimiter: None,
        contents: vec![],
    }];

    for token in input {
        match token {
            Token {
                kind: TokenKind::OpeningBracket(character),
                ..
            } => stack.push(TokenGroup {
                delimiter: Some(character),
                contents: vec![],
            }),
            Token {
                kind: TokenKind::ClosingBracket(character),
                ..
            } => {
                let last_stack_value = stack.pop().expect("Unmatched closing bracket (type 3)");
                stack
                    .last_mut()
                    .expect("Unmatched closing bracket (type 2)")
                    .contents
                    .push(LexerToken::Group(last_stack_value));
            }
            k => stack
                .last_mut()
                .expect("Unmatched closing parenthesis (type 1)")
                .contents
                .push(LexerToken::BasicToken(k)),
        }
    }

    if stack.len() != 1 {
        panic!(
            "Missing a closing bracket (type 4) Number required: {}",
            stack.len() as isize - 1
        );
    }

    stack
        .pop()
        .expect("Extra closing bracket found at end (type 5)")
}

use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct TokenGroup {
    pub delimiter: Option<char>,
    pub contents: Vec<LexerToken>,
}

impl TokenGroup {
    pub fn split_first(&self, kind: TokenKind) -> Option<(TokenGroup, Token, TokenGroup)> {
        let index = self
            .contents
            .iter()
            .enumerate()
            .find(|(index, i)| match i {
                LexerToken::BasicToken(k) => k.kind == kind,
                _ => false,
            })?
            .0;

        Some((
            TokenGroup {
                delimiter: self.delimiter,
                contents: self.contents[..index].to_vec(),
            },
            match self.contents[index].clone() {
                LexerToken::BasicToken(k) => Some(k),
                _ => None,
            }?,
            TokenGroup {
                delimiter: self.delimiter,
                contents: self.contents[index + 1..].to_vec(),
            },
        ))
    }

    pub fn split_all(&self, kind: TokenKind) -> Vec<TokenGroup> {
        return self
            .contents
            .split(|b| match b {
                LexerToken::BasicToken(b) => b.kind == kind,
                _ => false,
            })
            .map(|b| TokenGroup {
                delimiter: None,
                contents: b.to_vec(),
            })
            .collect();
    }

    pub fn contains(&self, kind: TokenKind) -> bool {
        return self.contents.iter().any(|b| match b {
            LexerToken::BasicToken(b) => b.kind == kind,
            _ => false,
        });
    }
}

#[derive(Debug, Clone)]
pub enum LexerToken {
    Group(TokenGroup),
    BasicToken(Token),
}

pub fn lex(input: Vec<Token>) -> TokenGroup {
    let mut stack: Vec<TokenGroup> = vec![
        TokenGroup {
            delimiter: None,
            contents: vec![],
        },
        TokenGroup {
            delimiter: Some(';'),
            contents: vec![],
        },
    ];

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
                kind: TokenKind::Semicolon,
                ..
            } => {
                let last_stack_value = stack.pop().expect("Unmatched closing bracket (type 6)");
                if last_stack_value.delimiter != Some(';') {
                    panic!("Unexpected ;, expected a {:?}, you may be missing a closing bracket (type 7)", last_stack_value.delimiter)
                }
                stack
                    .last_mut()
                    .expect("Unmatched closing bracket (type 8)")
                    .contents
                    .push(LexerToken::Group(last_stack_value));
                stack.push(TokenGroup {
                    delimiter: Some(';'),
                    contents: vec![],
                });
            }
            Token {
                kind: TokenKind::Comment,
                ..
            } => (),
            Token {
                kind: TokenKind::ClosingBracket(character),
                ..
            } => {
                let last_stack_value = stack.pop().expect("Unmatched closing bracket (type 3)");
                if last_stack_value.delimiter == None || last_stack_value.delimiter == Some(';') {
                    panic!(
                        "Expected a closing bracket to match {:?} but got {character}",
                        last_stack_value.delimiter
                    )
                }
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

    if stack.len() != 2 {
        panic!(
            "Missing a closing bracket (type 4) Number required: {}",
            stack.len() as isize - 1
        );
    }

    if stack.len() >= 2 && stack[1].contents.len() > 0 {
        panic!("Missing a ; at the end")
    }

    stack.remove(0)
}

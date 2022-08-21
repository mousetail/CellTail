use crate::errors::{self, SourceCodePosition};
use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct TokenGroup {
    pub delimiter: Option<char>,
    pub contents: Vec<LexerToken>,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl TokenGroup {
    pub fn split_first(&self, kind: TokenKind) -> Option<(TokenGroup, Token, TokenGroup)> {
        let index = self
            .contents
            .iter()
            .enumerate()
            .find(|(_, i)| match i {
                LexerToken::BasicToken(k) => k.kind == kind,
                _ => false,
            })?
            .0;

        let middle_element = match self.contents[index].clone() {
            LexerToken::BasicToken(k) => Some(k),
            _ => None,
        }?;

        Some((
            TokenGroup {
                delimiter: self.delimiter,
                contents: self.contents[..index].to_vec(),
                start: self.start,
                end: Some(middle_element.start),
            },
            middle_element.clone(),
            TokenGroup {
                delimiter: self.delimiter,
                contents: self.contents[index + 1..].to_vec(),
                start: Some(middle_element.end),
                end: self.end,
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
                start: b.first().and_then(|i| i.get_start()).or(self.start),
                end: b.last().and_then(|i| i.get_end()).or(self.end),
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

impl errors::SourceCodePosition for TokenGroup {
    fn get_start(&self) -> Option<usize> {
        self.start
    }
    fn get_end(&self) -> Option<usize> {
        self.end
    }
}

#[derive(Debug, Clone)]
pub enum LexerToken {
    Group(TokenGroup),
    BasicToken(Token),
}

impl errors::SourceCodePosition for LexerToken {
    fn get_start(&self) -> Option<usize> {
        match self {
            LexerToken::Group(k) => k.start,
            LexerToken::BasicToken(k) => Some(k.start),
        }
    }
    fn get_end(&self) -> Option<usize> {
        match self {
            LexerToken::Group(k) => k.end,
            LexerToken::BasicToken(k) => Some(k.end),
        }
    }
}

pub fn lex(input: Vec<Token>) -> errors::CellTailResult<TokenGroup> {
    let mut stack: Vec<TokenGroup> = vec![
        TokenGroup {
            delimiter: None,
            contents: vec![],
            start: Some(0),
            end: input.last().and_then(|i| Some(i.end)),
        },
        TokenGroup {
            delimiter: Some(';'),
            contents: vec![],
            start: Some(0),
            end: input.last().and_then(|i| Some(i.end)),
        },
    ];

    for token in input {
        match token {
            Token {
                kind: TokenKind::OpeningBracket(character),
                start,
                ..
            } => stack.push(TokenGroup {
                delimiter: Some(character),
                contents: vec![],
                start: Some(start),
                end: None,
            }),
            Token {
                kind: TokenKind::Semicolon,
                start,
                ..
            } => {
                let mut last_stack_value = stack.pop().expect("Unmatched closing bracket (type 6)");
                if last_stack_value.delimiter != Some(';') {
                    panic!("Unexpected ;, expected a {:?}, you may be missing a closing bracket (type 7)", last_stack_value.delimiter)
                }
                last_stack_value.end = Some(start);
                stack
                    .last_mut()
                    .expect("Unmatched closing bracket (type 8)")
                    .contents
                    .push(LexerToken::Group(last_stack_value));
                stack.push(TokenGroup {
                    delimiter: Some(';'),
                    contents: vec![],
                    start: Some(start),
                    end: None,
                });
            }
            Token {
                kind: TokenKind::Comment,
                ..
            } => (),
            Token {
                kind: TokenKind::ClosingBracket(character),
                start,
                ..
            } => {
                let mut last_stack_value = stack.pop().expect("Unmatched closing bracket (type 3)");
                last_stack_value.end = Some(start);
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

    Ok(stack.remove(0))
}

use crate::errors;
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
            },
            middle_element.clone(),
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

impl errors::SourceCodePosition for TokenGroup {
    fn get_start(&self) -> Option<usize> {
        self.contents.first().and_then(|i| i.get_start())
    }
    fn get_end(&self) -> Option<usize> {
        self.contents.last().and_then(|i| i.get_end())
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
            LexerToken::Group(k) => k.get_start(),
            LexerToken::BasicToken(k) => Some(k.start),
        }
    }
    fn get_end(&self) -> Option<usize> {
        match self {
            LexerToken::Group(k) => k.get_end(),
            LexerToken::BasicToken(k) => Some(k.end),
        }
    }
}

pub fn lex(input: Vec<Token>) -> errors::CellTailResult<TokenGroup> {
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
                    return Err(errors::CellTailError::new(
                        &last_stack_value,
                         format!("Unexpected ;, expected a {:?}, you may be missing a closing bracket (type 7)", last_stack_value.delimiter)));
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
        return Err(errors::CellTailError::new(
            &stack.pop(),
            format!(
                "Missing a closing bracket (type 4) Number required: {}",
                stack.len() as isize - 1
            ),
        ));
    }

    if stack.len() >= 2 && stack[1].contents.len() > 0 {
        Err(errors::CellTailError::new(
            stack.last().unwrap(),
            "Expected a semicolon at the end".to_owned(),
        ))?
    }

    Ok(stack.remove(0))
}

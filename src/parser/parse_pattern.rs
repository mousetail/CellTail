use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::parser::parse_expression;
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;
use crate::tokenizer::{Token, TokenKind};

pub(super) fn parse_as_pattern(input: TokenGroup) -> errors::CellTailResult<Pattern> {
    assert!(
        input.delimiter == Some('(') || input.delimiter == Some(';') || input.delimiter == None,
        "Unexpected input delimiter: {:?}",
        input.delimiter
    );

    if input.contents.len() == 1 {
        return Ok(match &input.contents[0] {
            LexerToken::Group(group) => parse_as_pattern(group.clone())?,
            LexerToken::BasicToken(Token {
                kind: TokenKind::Number,
                value,
                ..
            }) => Pattern::Literal(Literal::Number(value.parse().unwrap())),
            LexerToken::BasicToken(Token {
                kind: TokenKind::Identifier,
                value: v,
                ..
            }) => match v.as_str() {
                "N" => Pattern::Literal(Literal::Null),
                "_" => Pattern::Any,
                u => Pattern::Identifier(u.to_owned()),
            },
            LexerToken::BasicToken(Token {
                kind: TokenKind::String,
                value,
                ..
            }) => Pattern::Literal(Literal::new_string_literal(value.clone().as_bytes())),
            t => {
                return Err(errors::CellTailError::new(
                    &input,
                    format!("Unexpected token in match expression: {:?}", t),
                ));
            }
        });
    }

    if input.contains(TokenKind::Comma) {
        return Ok(Pattern::Tuple(
            input
                .split_all(TokenKind::Comma)
                .into_iter()
                .map(|i| parse_as_pattern(i))
                .collect::<errors::CellTailResult<_>>()?,
        ));
    }

    Ok(Pattern::Expression(parse_expression::parse_as_expression(
        input,
    )?))

    // Err(errors::CellTailError::new(
    //     &input,
    //     format!("Unexpected multi value expression in pattern: {:?}", input),
    // ))
}

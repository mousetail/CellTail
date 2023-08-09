fn parse_as_number(input: &TokenGroup) -> errors::CellTailResult<isize> {
    if input.contents.len() == 2 {
        if let (
            LexerToken::BasicToken(Token {
                kind: TokenKind::Operator('-'),
                ..
            }),
            LexerToken::BasicToken(Token {
                kind: TokenKind::Number,
                value,
                ..
            }),
        ) = (&input.contents[0], &input.contents[1])
        {
            Ok(-value.parse().map_err(|k| {
                errors::CellTailError::new(
                    input,
                    format!("Failed to parse negative number literal: {k:?}"),
                )
            })?)
        } else {
            Err(errors::CellTailError::new(
                input,
                format!("Failure parsing as negative number literal, expected 2 tokens with the left being '-` but got a {:?}", input.contents[0]),
            ))
        }
    } else if input.contents.len() == 1 {
        if let LexerToken::BasicToken(Token {
            kind: TokenKind::Number,
            value,
            ..
        }) = &input.contents[0]
        {
            value.parse().map_err(|k| {
                errors::CellTailError::new(
                    input,
                    format!("Failed to parse positive number literal: {k:?}"),
                )
            })
        } else {
            Err(errors::CellTailError::new(
                input,
                "Invalid positive number literal".to_owned(),
            ))
        }
    } else {
        Err(errors::CellTailError::new(
            input,
            format!("Empty or too long number literal: {:?}", input.contents),
        ))
    }
}

fn parse_single_attribute(
    name: &str,
    value: TokenGroup,
    attrs: &mut attributes::Attributes,
) -> errors::CellTailResult<()> {
    match name {
        "I" | "Input" => {
            if value.contains(TokenKind::Comma) {
                let result = value.split_all(TokenKind::Comma).iter().map(
                    parse_as_number
                ).collect::<errors::CellTailResult<Vec<_>>>()?;

                Ok(attrs.input_mode = attributes::InputSource::Constant(result))
            } else if let [
                LexerToken::BasicToken(Token{
                kind: TokenKind::Identifier,
                value: input_type,
                ..
            }), LexerToken::BasicToken(Token{
                kind: TokenKind::Identifier,
                value: input_format,
                ..
            }),
            ] = value.contents.as_slice() {
                let input_format = match input_format.to_uppercase().as_str() {
                    "N" | "NUMBERS" | "NRS" => attributes::IOFormat::Numbers,
                    "C" | "CHARACTERS" | "CHARS" => attributes::IOFormat::Characters,
                    _ => Err(errors::CellTailError::new(&value, "Invalid value for input format, expected one of 'NUMBERS' or 'CHARS'".to_owned()))?
                };

                match input_type.to_uppercase().as_str() {
                    "I" | "STDIN" => Ok(attrs.input_mode = attributes::InputSource::StdIn(input_format)),
                    "C" | "CMD" | "COMMANDLINEARGUMENTS" | "ARGS" | "ARGV" | "A" => Ok(attrs.input_mode = attributes::InputSource::Arg(input_format)),
                    _ => Err(errors::CellTailError::new(&value, "Invalid value for input mode, expected one of 'STDIN', 'CMD'".to_owned()))
                }
            } else if let [
                LexerToken::BasicToken(Token{
                    kind: TokenKind::String,
                    value: input_format,
                    ..
                })
            ] = value.contents.as_slice() {
                Ok(attrs.input_mode = attributes::InputSource::Constant(input_format.chars().map(|i| i as u32 as isize).collect()))
            }
            else {
                if let Ok(number) = parse_as_number(&value) {
                    Ok(attrs.input_mode = attributes::InputSource::Constant(vec![number]))
                } else {
                    Err(errors::CellTailError::new(&value, "Invalid attribute value for attribute \"input\", expected 2 words or a comma seperated list of numbers".to_owned()))
                }
            }
        }
        "O" | "Output" => {
            if value.contents.len() != 1 {
                return Err(errors::CellTailError::new(&value, "Invalid length for property \"output\"".to_owned()))
            }

            match &value.contents[0] {
                LexerToken::BasicToken(Token {
                    kind: TokenKind::Identifier,
                    value: val,
                    ..
                }) => {
                   match val.to_lowercase().as_str() {
                        "c" | "chars" | "characters"  => Ok(attrs.output_mode = attributes::IOFormat::Characters),
                        "n" | "d" | "numbers" | "decimal" => Ok(attrs.output_mode = attributes::IOFormat::Numbers),
                        _ => Err(errors::CellTailError::new(&value, "Invalid output mode, must be one of \"characters\" or \"numbers\"".to_owned()))
                    }
                }
                _ => Err(errors::CellTailError::new(&value, "Invalid type for property \"output\", note: must be token, no parenthesis allowed here".to_owned()))
            }
        }
        "D" | "Debug" => {
            match &value.contents[0] {
                LexerToken::BasicToken(Token {
                    kind: TokenKind::Identifier,
                    value: val,
                    ..
                }) => {
                   match val.to_lowercase().as_str() {
                        "t" | "y" | "true" | "yes"  => Ok(attrs.debug = true),
                        "n" | "f" | "no" | "false" => Ok(attrs.debug = false),
                        _ => Err(errors::CellTailError::new(&value, "Invalid debug mode, must be one of \"characters\" or \"numbers\"".to_owned()))
                    }
                }
                _ => Err(errors::CellTailError::new(&value, "Invalid type for property \"debug\", note: must be token, no parenthesis allowed here".to_owned()))
            }
        }
        "M" | "Max" | "MaxIterations" => {
            Ok(attrs.max_iterations = Some(parse_as_number(&value)?))
        },
        m => {
            Err(errors::CellTailError::new(&value, format!("Unexpected property name {}, expected one of 'Input', 'I', 'Output', 'O', 'Debug', 'D'", m)))
        }
    }
}

use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::runtime::attributes;
use crate::tokenizer::{Token, TokenKind};

pub(super) fn parse_attribute(
    input: TokenGroup,
    attributes: &mut attributes::Attributes,
) -> errors::CellTailResult<()> {
    if let Some((name, _, value)) = input.split_first(TokenKind::Equals) {
        if name.contents.len() != 1 {
            Err(errors::CellTailError::new(
                &name,
                "Too long attribute name".to_owned(),
            ))?
        };

        if let LexerToken::BasicToken(Token {
            kind: TokenKind::Identifier,
            value: name,
            ..
        }) = &name.contents[0]
        {
            parse_single_attribute(name, value, attributes)
        } else {
            Err(errors::CellTailError::new(
                &input,
                "Expected a attribute name".to_owned(),
            ))
        }
    } else {
        Err(errors::CellTailError::new(
            &input,
            "Invalid attribute definition".to_owned(),
        ))
    }
}

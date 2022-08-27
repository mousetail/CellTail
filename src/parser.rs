use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::runtime::attributes;
use crate::runtime::expression::{BinaryOperator, Expression, UnaryOperator};
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;
use crate::runtime::pattern_list::PatternList;
use crate::tokenizer::{Token, TokenKind};
use std::collections::HashMap;

pub struct Program {
    pub functions: HashMap<String, PatternList>,
    pub rules: PatternList,
    pub attributes: attributes::Attributes,
}

impl Program {
    fn new() -> Program {
        Program {
            functions: HashMap::new(),
            rules: PatternList(Vec::new()),
            attributes: attributes::Attributes::new(),
        }
    }

    fn add_function_rule(&mut self, function_name: String, rule: (Pattern, Expression)) {
        if let Some(pattern_list) = self.functions.get_mut(&function_name) {
            pattern_list.0.push(rule);
        } else {
            self.functions
                .insert(function_name, PatternList(vec![rule]));
        }
    }

    fn add_rule(&mut self, rule: (Pattern, Expression)) {
        self.rules.0.push(rule);
    }
}

fn parse_as_expression(input: TokenGroup) -> errors::CellTailResult<Expression> {
    assert!(
        input.delimiter == Some('(') || input.delimiter == Some(';') || input.delimiter == None,
        "Unexpected input delimiter: {:?}",
        input.delimiter
    );
    if input.contents.len() == 1 {
        return Ok(match &input.contents[0] {
            LexerToken::Group(group) => parse_as_expression(group.clone())?,
            LexerToken::BasicToken(Token {
                kind: TokenKind::Number,
                value,
                ..
            }) => Expression::Literal(Literal::Number(value.parse().unwrap())),
            LexerToken::BasicToken(Token {
                kind: TokenKind::Identifier,
                value,
                ..
            }) => Expression::Variable(value.clone()),
            LexerToken::BasicToken(Token {
                kind: TokenKind::String,
                value,
                ..
            }) => Expression::Literal(Literal::new_string_literal(value.clone().as_bytes())),
            t => {
                return Err(errors::CellTailError::new(
                    &input,
                    format!("Unexpected token in match expression: {:?}", t),
                ))
            }
        });
    }

    if input.contains(TokenKind::Comma) {
        return Ok(Expression::Tuple(
            input
                .split_all(TokenKind::Comma)
                .into_iter()
                .map(parse_as_expression)
                .collect::<errors::CellTailResult<Vec<_>>>()?,
        ));
    }

    for operator in [('-', UnaryOperator::Neg), ('!', UnaryOperator::Not)] {
        if let Some(LexerToken::BasicToken(Token {
            kind: TokenKind::Operator(op),
            ..
        })) = input.contents.first()
        {
            if *op == operator.0 {
                return Ok(Expression::UnaryOperator(
                    operator.1,
                    Box::new(parse_as_expression(TokenGroup {
                        delimiter: None,
                        contents: input.contents[1..].to_vec(),
                    })?),
                ));
            }
        }
    }

    for operator in [
        ('+', BinaryOperator::Add),
        ('-', BinaryOperator::Subtract),
        ('*', BinaryOperator::Multiply),
        ('/', BinaryOperator::Divide),
        ('&', BinaryOperator::And),
        ('|', BinaryOperator::Or),
        ('^', BinaryOperator::Xor),
        ('%', BinaryOperator::Mod),
    ] {
        if let Some((part1, _op, part2)) = input.split_first(TokenKind::Operator(operator.0)) {
            return Ok(Expression::BinaryOperator(
                operator.1,
                Box::new(parse_as_expression(part1)?),
                Box::new(parse_as_expression(part2)?),
            ));
        }
    }

    if input.contents.len() == 2 {
        if let LexerToken::BasicToken(Token {
            kind: TokenKind::Identifier,
            value,
            ..
        }) = &input.contents[0]
        {
            return Ok(Expression::FunctionCall(
                value.clone(),
                Box::new(parse_as_expression(TokenGroup {
                    delimiter: None,
                    contents: vec![input.contents[1].clone()],
                })?),
            ));
        }
    }

    Err(errors::CellTailError::new(
        &input,
        format!("Invalid expression"),
    ))
}

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
                "Invalid negative number literal".to_owned(),
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
            } else {
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
        m => {
            Err(errors::CellTailError::new(&value, format!("Unexpected property name {}, expected one of 'Input', 'I', 'Output', 'O', 'Debug', 'D'", m)))
        }
    }
}

fn parse_attribute(
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

fn parse_as_pattern(input: TokenGroup) -> errors::CellTailResult<Pattern> {
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
            }) if v == "N" => Pattern::Literal(Literal::Null),
            LexerToken::BasicToken(Token {
                kind: TokenKind::Identifier,
                value,
                ..
            }) => Pattern::Identifier(value.clone()),
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

    Ok(Pattern::Expression(parse_as_expression(input)?))

    // Err(errors::CellTailError::new(
    //     &input,
    //     format!("Unexpected multi value expression in pattern: {:?}", input),
    // ))
}

pub fn parse(input: TokenGroup) -> errors::CellTailResult<Program> {
    let mut out = Program::new();
    for statement in input.contents {
        if let LexerToken::Group(group) = statement {
            if group.contains(TokenKind::Equals) {
                parse_attribute(group, &mut out.attributes)?;
            } else if let Some((pattern, _operator, expression)) =
                group.split_first(TokenKind::Colon)
            {
                if let LexerToken::BasicToken(
                    t @ Token {
                        kind: TokenKind::Identifier,
                        ..
                    },
                ) = &pattern.contents[0]
                {
                    if t.value == "fn" {
                        let function_name = if let LexerToken::BasicToken(
                            t @ Token {
                                kind: TokenKind::Identifier,
                                ..
                            },
                        ) = &pattern.contents[1]
                        {
                            t.value.clone()
                        } else {
                            return Err(errors::CellTailError::new(
                                &pattern,
                                "Expected a function name".to_owned(),
                            ));
                        };
                        out.add_function_rule(
                            function_name,
                            (
                                parse_as_pattern(TokenGroup {
                                    delimiter: None,
                                    contents: pattern.contents[1..].to_vec(),
                                })?,
                                parse_as_expression(expression)?,
                            ),
                        );

                        continue;
                    }
                }

                out.add_rule((parse_as_pattern(pattern)?, parse_as_expression(expression)?))
            } else {
                return Err(errors::CellTailError::new(
                    &group,
                    "Missing : seperating pattern from expression".to_owned(),
                ));
            }
        } else {
            return Err(errors::CellTailError::new(
                &statement,
                "Invalid top-level statement".to_owned(),
            ));
        }
    }

    return Ok(out);
}

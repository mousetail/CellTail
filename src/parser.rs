use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::runtime::expression::{BinaryOperator, Expression, UnaryOperator};
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;
use crate::runtime::pattern_list::PatternList;
use crate::tokenizer::{Token, TokenKind};
use std::collections::HashMap;

pub struct Program {
    pub functions: HashMap<String, PatternList>,
    pub rules: PatternList,
    pub attributes: HashMap<String, Literal>,
}

impl Program {
    fn new() -> Program {
        Program {
            functions: HashMap::new(),
            rules: PatternList(Vec::new()),
            attributes: HashMap::new(),
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

    fn add_attribute(&mut self, name: String, value: Literal) {
        self.attributes.insert(name, value);
    }
}

fn parse_as_expression(input: TokenGroup) -> Expression {
    assert!(
        input.delimiter == Some('(') || input.delimiter == Some(';') || input.delimiter == None,
        "Unexpected input delimiter: {:?}",
        input.delimiter
    );
    if input.contents.len() == 1 {
        return match &input.contents[0] {
            LexerToken::Group(group) => parse_as_expression(group.clone()),
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
            t => panic!("Unexpected token in match expression: {:?}", t),
        };
    }

    if input.contains(TokenKind::Comma) {
        return Expression::Tuple(
            input
                .split_all(TokenKind::Comma)
                .into_iter()
                .map(parse_as_expression)
                .collect(),
        );
    }

    for operator in [('-', UnaryOperator::Neg), ('!', UnaryOperator::Not)] {
        if let Some(LexerToken::BasicToken(Token {
            kind: TokenKind::Operator(op),
            ..
        })) = input.contents.first()
        {
            if *op == operator.0 {
                return Expression::UnaryOperator(
                    operator.1,
                    Box::new(parse_as_expression(TokenGroup {
                        delimiter: None,
                        contents: input.contents[1..].to_vec(),
                        start: input.start,
                        end: input.end,
                    })),
                );
            }
        }
    }

    for operator in [
        ('+', BinaryOperator::Add),
        ('-', BinaryOperator::Subtract),
        ('*', BinaryOperator::Multiply),
        ('/', BinaryOperator::Divide),
    ] {
        if let Some((part1, _op, part2)) = input.split_first(TokenKind::Operator(operator.0)) {
            return Expression::BinaryOperator(
                operator.1,
                Box::new(parse_as_expression(part1)),
                Box::new(parse_as_expression(part2)),
            );
        }
    }

    panic!("Unexpected token: {:?}", input);
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

    Err(errors::CellTailError::new(
        &input,
        format!("Unexpected multi value expression in pattern: {:?}", input),
    ))
}

pub fn parse(input: TokenGroup) -> errors::CellTailResult<Program> {
    let mut out = Program::new();
    for statement in input.contents {
        if let LexerToken::Group(group) = statement {
            if let Some((pattern, _operator, expression)) = group.split_first(TokenKind::Colon) {
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
                                    start: Some(t.end),
                                    end: group.end,
                                })?,
                                parse_as_expression(expression),
                            ),
                        );

                        continue;
                    }
                }

                out.add_rule((parse_as_pattern(pattern)?, parse_as_expression(expression)))
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

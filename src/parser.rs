use crate::lexer::{LexerToken, TokenGroup};
use crate::runtime::expression::{BinaryOperator, Expression};
use crate::runtime::literal::Literal;
use crate::runtime::pattern::Pattern;
use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Statement {
    Ops(String),
    Function(String, Pattern, Expression),
    Rule(Pattern, Expression),
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

    todo!();
}

fn parse_as_pattern(input: TokenGroup) -> Pattern {
    assert!(
        input.delimiter == Some('(') || input.delimiter == Some(';') || input.delimiter == None,
        "Unexpected input delimited: {:?}",
        input.delimiter
    );

    if input.contents.len() == 1 {
        return match &input.contents[0] {
            LexerToken::Group(group) => parse_as_pattern(group.clone()),
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
            t => panic!("Unexpected token in match expression: {:?}", t),
        };
    }

    if input.contains(TokenKind::Comma) {
        return Pattern::Tuple(
            input
                .split_all(TokenKind::Comma)
                .into_iter()
                .map(parse_as_pattern)
                .collect(),
        );
    }
    todo!();
}

pub fn parse(input: TokenGroup) -> Vec<Statement> {
    let mut out = vec![];
    for statement in input.contents {
        if let LexerToken::Group(group) = statement {
            if let Some((pattern, _operator, expression)) = group.split_first(TokenKind::Colon) {
                out.push(Statement::Rule(
                    parse_as_pattern(pattern),
                    parse_as_expression(expression),
                ))
            } else {
                panic!("Statement missing ':' token")
            }
        } else {
            panic!("Unexpected statement type");
        }
    }

    return out;
}

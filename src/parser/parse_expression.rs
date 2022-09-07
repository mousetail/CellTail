use crate::errors;
use crate::lexer::{LexerToken, TokenGroup};
use crate::parser::parse_array::parse_array;
use crate::runtime::expression::{BinaryOperator, Expression, UnaryOperator};
use crate::runtime::literal::Literal;
use crate::tokenizer::{Token, TokenKind};

pub(super) fn parse_as_expression(input: TokenGroup) -> errors::CellTailResult<Expression> {
    if input.delimiter == Some('[') {
        return parse_array(
            input,
            parse_as_expression,
            |a, b| Ok(Expression::Tuple(vec![a, b])),
            Expression::Literal(Literal::Null),
        );
    }

    if !(input.delimiter == Some('(') || input.delimiter == Some(';') || input.delimiter == None) {
        return Err(errors::CellTailError::new(
            &input,
            format!("Unexpected input delimiter: {:?}", input.delimiter),
        ));
    }
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
            }) => match value.as_str() {
                "N" => Expression::Literal(Literal::Null),
                "_" => Err(errors::CellTailError::new(
                    &input.contents[0],
                    "'_' is not a valid variable name\nHelp: _ indicates discarding  a value. Thus there can never be a value assigned to _.".to_owned(),
                ))?,
                _ => Expression::Variable(value.clone()),
            },
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
        // ('&', BinaryOperator::And),
        // ('|', BinaryOperator::Or),
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
